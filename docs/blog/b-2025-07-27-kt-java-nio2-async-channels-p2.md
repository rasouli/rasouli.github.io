---
title: Kotlin Coroutines and Java NIO 2 Asynchronous Channels (Part II)
slug: b-2025-07-27-kt-java-nio2-async-channels-p2
date: 2025-07-27 00:00:00
publish: true
tags: 
  - java
  - kotlin
categories:
  - NIO
  - Java
  - Kotlin
---



We showed that using Kotlin Coroutines and its ecosystem, we can write much more maintainable code when taking advantage of Java NIO 2 Asynchronous Channels. This blog post is the follow-up and last part of [previous](blog/b-2025-05-24-kt-java-nio2-async-channels-p1.md) blog post where we managed to write a very simple Asynchronous and Non-blocking Java NIO server using [AsynchronousServerSocketChannel](https://docs.oracle.com/javase/8/docs/api/java/nio/channels/AsynchronousServerSocketChannel.html). Now in this part we will leverage Kotlin coroutines and rewrite the whole thing. The end result will be interesting in that we end up with a more maintainable code!

<!-- more -->

## Recap
Previously we ended up with [following](https://github.com/rasouli/blog-codes/blob/88c88997407a3754a84a99db76a333f31318dd2b/jvm/kotlin-java-nio/kotlin-java-nio/src/main/kotlin/rs/reza/pub/nioserver/BasicAsyncServer.kt) code, where we wrote all sort of callbacks inside our `CompletionHandlers`.

We can immediately see the obvious: 
    1. Writing callback based code makes understanding the code hard.
    2. Managing the state across callback is harder :D

## Somewhat quick re-write
Let's quickly re-write the previous server example to build the base for using Kotlin coroutines:

```kotlin
//...
    private val asyncChannelGroupThreadPool = Executors.newFixedThreadPool(2)
    private val asyncChannelGroupDispatcher = asyncChannelGroupThreadPool.asCoroutineDispatcher()

    @JvmStatic
    fun main(args: Array<String>) {
        runBlocking {
            run()
        }
    }

    suspend fun run() = coroutineScope {
        val address = InetSocketAddress(HOST, PORT)
        // we can ignore the cost of setting up the asynchronous channel group.
        val asynchronousChannelGroup = AsynchronousChannelGroup.withThreadPool(asyncChannelGroupThreadPool)
        val asyncServerSocketChannel = listen(address = address, group = asynchronousChannelGroup)

        asyncServerSocketChannel.use { serverSocketChannel ->
            while (serverSocketChannel.isOpen) {
                try {
                    // 1. we would use dedicated dispatcher for I/O operations such as accept, read and write. 
                    val clientChannel = withContext(asyncChannelGroupDispatcher) {
                        // ... in next section we will look into how we implement this.
                        serverSocketChannel.acceptAsync()
                    }
                    logger.info("accepted a new connection from client at ${clientChannel.remoteAddress}")
                    launch {
                        val coroutineName = CoroutineName("client_${clientChannel.remoteAddress}")
                        // 2. we need to rationalize on this choice of dispatcher.
                        withContext(asyncChannelGroupDispatcher + coroutineName) {
                            processClientChannel(clientChannel)
                        }
                    }
                } catch (ex: Exception) {
                    logger.error("exception while accepting new connections", ex)
                }
            }
        }
    }
 //...

```

This is good amount of code to digest for the beginning. There are a couple of interesting choices we need/wanted to make:

### First, Using dedicated Dispatcher to scope I/O operations 
Thanks to:

```kotlin
    private val asyncChannelGroupDispatcher = asyncChannelGroupThreadPool.asCoroutineDispatcher()
```

We can explicitly re-use the underlying Thread Pool to spawn coroutines that deal with Non-blocking and async I/O operations. I really like this feature mainly because it makes it super clear in combination with `withContext` to specify on which Thread Pool Kotlin coroutines handling I/O operations should be managed.

As you can see we are using some sort of magic to seemingly use a suspendable coroutine to accept incoming connections, we explicitly define where coroutine (which we will explore how it works next) should perform that action:

```kotlin
val clientChannel = withContext(asyncChannelGroupDispatcher) {
    serverSocketChannel.acceptAsync()
}
```

### Secondly, Using same dispatcher for all interaction with `AsynchronousSocketChannel`
Following up previous point, then we make this (kind of a discipline) that if anywhere that we are beginning to deal with read / write on the AsynchronousSocketChannel, we would use the same `asyncChannelGroupDispatcher` dispatcher.

Honestly this might be trivial, but I wanted to think a little about this: if after accepting a connection, I want to launch some sort of coroutine that supervises reading and writing (basically managing the client connection) what dispatcher should I use here? `Dispatcher.Default`, `Dispatcher.IO` or same beloved `asyncChannelGroupDispatcher`. If I confused you, you are right. I wanted to think what dispatcher to replace the question marks with:

```kotlin
// ...
    val coroutineName = CoroutineName("client_${clientChannel.remoteAddress}")
    withContext( ??? + coroutineName) {
        processClientChannel(clientChannel)
    }
//  ...
```

This `processClientChannel` is going to supervise the `AsynchronousSocketChannel` that has just been created by our client. So then the question becomes on which coroutine dispatcher we should schedule this coroutine for execution?
here follows the rationale why or why not to choose a certain type of dispatcher:

* `Dispatchers.IO`: This dispatcher is for genuinely blocking I/O. Our `AsynchronousSocketChannel` plays a non-blocking game, so using `Dispatchers.IO` here would just snag a thread for no good reason. It's a conceptual mismatch.

* `Dispatchers.Default`: This one's for heavy CPU lifting, not for orchestrating I/O. Our client channel's setup isn't CPU-bound, so launching it here would misuse resources or at very least is again a conceputal mismatch.

* (the dedicated) `asyncChannelGroupDispatcher`: This custom dispatcher is explicitly tied to our `AsynchronousChannelGroup`'s thread pool. It ensures the client handler's vital, non-blocking I/O flow kicks off and continues on threads purpose-built for it, providing seamless integration and efficiency right from the get-go.

Based on the above thoughts, I think it is a safe bet to use `asyncChannelGroupDispatcher` here.

## Accepting connection asynchronously as if it was a suspendable  method call
So, I kept you waiting for the real deal until this very moment: How can we make that ugly callback code (in part I) to look like a normal suspendable call in Kotlin? the answer simply is: `suspendCancellableCoroutine`

## `suspendCancellableCoroutine`

A `suspendCancellableCoroutine` allows us to wrap a callback based API and isolate it within a suspendable call, so from a high level code it looks as if we are just simply introducing a suspension point to our coroutine. 

Let's look at the general idea of `suspendCancellableCoroutine` in an example scenario: Imagine someone throws some ugly callback-based client library (like this one) at you:

```kotlin
object SomeAPI {
    fun getFoo(
        onSuccess: (Foo) -> Unit,
        onFailure: (Exception) -> Unit
    ) {
        // ....
    }

    fun cleanUpResources() = println("some clean up happenning here...")
}
```

The `getFoo` the method, asks for two callback functions:
 * `onSuccess` : to be called when the `Foo` is actually fetched.
 * `onFailure` : to be called when fetching `Foo` fails with an exception, kindly passing on the exception to the callback for failure scenario.

All these and they are kind enough to provide `cleanUpResources` method to be called at some point where you just want to clean up all resources (e.g. thread pools, connection pools, etc) used by the client library.

what you can do with `suspendCancellableCoroutine` is to isolate this ugly callback API code to use it with your other (handsome) Kotlin coroutines, like this:

```kotlin
  object MyCode {
        suspend fun getFoo(): Foo = suspendCancellableCoroutine { continuation ->
            SomeAPI.getFoo(
                onSuccess = { foo -> continuation.resume(foo) },
                onFailure = { continuation.resumeWithException(it) }
            )

            continuation.invokeOnCancellation {
                SomeAPI.cleanUpResources()
            }
        }
    }
```

This example provides a clear, three-fold demonstration of bridging traditional asynchronous APIs with the power of Kotlin Coroutines. First, it illustrates the wrapping of callback-based APIs. The `SomeAPI.getFoo` method, a typical asynchronous function that delivers its result or error via distinct onSuccess and onFailure callbacks, is transformed. Inside MyCode, the getFoo suspending function uses suspendCancellableCoroutine to immediately suspend the calling coroutine. This allows `SomeAPI.getFoo` to be invoked, with its callbacks seamlessly wired to `continuation.resume(foo)` for success or `continuation.resumeWithException(it)` for failure, effectively converting an imperative callback pattern into a sequential, suspendable call.

Second, the example highlights seamless error and result propagation. By mapping SomeAPI's `onSuccess` to `continuation.resume` and `onFailure` to `continuation.resumeWithException`, the coroutine system inherently handles the outcome. A successful `Foo` result flows directly as the return value of the suspend function, while any Exception from the underlying API is rethrown, allowing standard try-catch blocks in the calling coroutine to manage errors. This eliminates the need for manual error checking or complex callback chaining.

Finally, a critical aspect of robust asynchronous programming is there right in front of us: correct resource cleanup upon cancellation. The `continuation.invokeOnCancellation` block is important. Should the coroutine calling `getFoo` be cancelled before `SomeAPI.getFoo` completes, `SomeAPI.cleanUpResources()` is guaranteed to execute. This mechanism ensures that any resources held or operations initiated by the underlying `SomeAPI` are properly released or halted, preventing leaks and maintaining system stability even when long-running operations are unexpectedly terminated.

Lots stuff to take in, regardless we end-up with a very nice way to call that call-back based API:

```kotlin
suspend fun oneFoo() : Foo {
    try {
        return MyCode.getFoo();
    } catch(...) {
        //...
    }
}
```

### Wrapping the Accept `CompletionHandler`

Let's apply what we learned in the previous section now for creating a suspending extension function for accepting incoming client connections:

```kotlin 
package rs.reza.pub.rs.reza.blog.nioserver.handlers

import kotlinx.coroutines.suspendCancellableCoroutine
import java.nio.channels.AsynchronousServerSocketChannel
import java.nio.channels.AsynchronousSocketChannel
import java.nio.channels.CompletionHandler
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException


suspend fun AsynchronousServerSocketChannel.acceptAsync(): AsynchronousSocketChannel =
    suspendCancellableCoroutine { continuation ->
        accept(
            Unit,
            object : CompletionHandler<AsynchronousSocketChannel, Unit> {
                override fun completed(clientChannel: AsynchronousSocketChannel, attachment: Unit) {
                    continuation.resume(clientChannel)
                }

                override fun failed(exception: Throwable, attachment: Unit) {
                    continuation.resumeWithException(exception)
                }
            })

        continuation.invokeOnCancellation { close() }
    }
```

In the code above: 
* we are wrapping a `CompletionHandler` that can be registered for accepting incoming client connections with `AsynchronousServerSocketChannel`. We can spot a `CompletionHandler` for accepting connection that has following type params:

 ```kotlin
 CompletionHandler<AsynchronousSocketChannel, Unit>
 ```
* Since we really are interested to pass the `AsynchronousSocketChannel` to the continuation, we really don't need to maintain any special state in attachment, hence the attachment has type `Unit`. This would bring a huge simplification because the state that now needs to be maintained will live in higher level code where it is dealing with suspendable function calls!
  
* As you could guess, we take advantage of `continuation.resumeWithException` to pass on exception to the continuation: Any exception that occurs while we were waiting for new incoming connection now can be handled in a try-catch block in the high level code.
  
* With `continuation.invokeOnCancellation` we are basically saying if the coroutine got cancelled to just call `AsynchronousServerSocketChannel::close` on the way out.


## Beyond Callback
Now, let's dive into `processClientChannel`:

```kotlin
    private suspend fun processClientChannel(
        clientSocketChannel: AsynchronousSocketChannel
    ) = coroutineScope {
        val bufferChannel = Channel<ByteBuffer>(capacity = Channel.UNLIMITED, onBufferOverflow = BufferOverflow.SUSPEND)
        clientSocketChannel.use {
            try {
                val readCoroutine = launch {
                    withContext(asyncChannelGroupDispatcher) {
                        read(clientSocketChannel, bufferChannel)
                    }
                }
                val writeCoroutine = launch {
                    withContext(asyncChannelGroupDispatcher) {
                        write(clientSocketChannel, bufferChannel)
                    }
                }

                joinAll(readCoroutine, writeCoroutine)
            } catch (ex: CancellationException) {
                logger.warning("Coroutine ${coroutineContext[CoroutineName]} got cancelled.", ex)
                throw ex
            } catch (ex: IOException) {
                logger.error(
                    "I/O exception while processing client connection at ${clientSocketChannel.remoteAddress}",
                    ex
                )
            } catch (ex: Exception) {
                logger.error(
                    "unexpected exception while processing client connection at ${clientSocketChannel.remoteAddress}",
                    ex
                )
            }
        }
    }
```

we are using `withContext(asyncChannelGroupDispatcher)` to signal which Dispatcher we desire our coroutines responsible for reading and writing to `AsynchronousSocketChannel` should be scheduled. Additionally we use channels to share buffers from coroutines that `read` to coroutines that `write` on the same channel.

And here is how `read` and `write` functions are implemented (notice the usage of `readAsync` and `writeAsync`), they should look familiar to you:

```kotlin
    private suspend fun write(
        clientChannel: AsynchronousSocketChannel,
        channel: Channel<ByteBuffer>
    ) {
        for (buf in channel) {
            Utils.changeLetterCase(buf)
            while (buf.hasRemaining()) {
                buf.flip()
                val numsWritten = clientChannel.writeAsync(buf)
                if (numsWritten == -1) {
                    logger.info("client channel closed at ${clientChannel.remoteAddress}")
                    return
                }
            }
        }
    }

    private suspend fun read(
        clientChannel: AsynchronousSocketChannel,
        channel: Channel<ByteBuffer>
    ) {
        while (clientChannel.isOpen) {
            val buffer = ByteBuffer.allocate(80)
            val readBytes = clientChannel.readAsync(buffer)
            if (readBytes == -1) {
                logger.info("client at ${clientChannel.remoteAddress} closed the connection.")
                return
            }

            if (readBytes > 0) {
                channel.send(buffer)
            }
        }
    }

```

### `readAsync`

here is `readAsync` working under the hood:

```kotlin
suspend fun AsynchronousSocketChannel.readAsync(buffer: ByteBuffer): Int =
    suspendCancellableCoroutine { continuation ->
        read(
            buffer,
            Unit,
            object : CompletionHandler<Int, Unit> {
                override fun completed(bytesRead: Int, attachment: Unit) {
                   continuation.resume(bytesRead)
                }

                override fun failed(exception: Throwable, attachment: Unit) {
                    continuation.resumeWithException(exception)
                }

            }
        )

        continuation.invokeOnCancellation { close() }
    }

```

Please note that we only can return the number of bytes read `readAsync`.

### `writeAsync`
Finally, the `writeAsync`:

```kotlin

suspend fun AsynchronousSocketChannel.writeAsync(buffer: ByteBuffer): Int =
    suspendCancellableCoroutine { continuation ->
        write(
            buffer,
            Unit,
            object : CompletionHandler<Int, Unit> {
                override fun completed(bytesWritten: Int, attachment: Unit) {
                    continuation.resume(bytesWritten)
                }

                override fun failed(exception: Throwable, attachment: Unit) {
                    continuation.resumeWithException(exception)
                }

            }
        )

        continuation.invokeOnCancellation { close() }
    }

```

Please note that we only can return the number of bytes written to the channel from `writeAsync`.


## Full Code
You can find the whole code we developed in second part [here](https://github.com/rasouli/blog-codes/blob/8596668f6ae6e0a057ffe47084ecef457984e086/jvm/kotlin-java-nio/kotlin-java-nio/src/main/kotlin/rs/reza/pub/nioserver/AsyncServer.kt).


## Final words
We came really far: We figured out how to use `AsynchronousSocketChannel`, `CompletionHandler` and Attachments in Kotlin and then learned how to build powerful abstraction on top of them and use it within Kotlin Coroutine ecosystem. 

I think what we learned is that we can write far more maintainable code in Kotlin when it comes down to dealing with Callback based API not only that, but we can easily extend the code to add more functionality such as buffer management, resource sharing, statistics etc which would have been far more painful with Attachments and `CompletionHandler`s. 