---
title: Kotlin Coroutines and Java NIO 2 Asynchronous Channels (Part I)
slug: b-2025-05-24-kt-java-nio2-async-channels-p1
date: 2025-05-24 00:00:00
publish: true
tags: 
  - java
  - kotlin
categories:
  - NIO
  - Java
  - Kotlin
---


Thanks to Kotlin Coroutines and its ecosystem, we can write much more maintainable code when taking advantage of Java NIO 2 Asynchronous Channels. This two part blog post will showcase how we can fuse Java NIO 2 [AsynchronousServerSocketChannel](https://docs.oracle.com/javase/7/docs/api/java/nio/channels/AsynchronousServerSocketChannel.html) and Kotlin Coroutines together to write a maintainable non-blocking asynchronous network applications.

<!-- more -->

## Part I: A Callback-based Asynchronous Non-blocking Server using AsynchronousServerSocketChannel in Kotlin

A typical implementation of an Asynchronous and Non-Blocking network applications using Java NIO 2 API is basically implemented in two approaches using `AsynchronousServerSocketChannel`:

* **Using `AsynchronousChannel` Future-based API**: using a future-based API such as future-based [`read`](https://docs.oracle.com/javase/7/docs/api/java/nio/channels/AsynchronousSocketChannel.html#read(java.nio.ByteBuffer)) 
* **Registering I/O callbacks**: Using Callbacks which implement the [`CompletionHandler`](https://docs.oracle.com/javase/7/docs/api/java/nio/channels/CompletionHandler.html) interface and are registeretd with a callback based method such as [`read`](https://docs.oracle.com/javase/7/docs/api/java/nio/channels/AsynchronousSocketChannel.html#read(java.nio.ByteBuffer,%20A,%20java.nio.channels.CompletionHandler))

For our goal in this blog post, we will be focusing on the callback based API in Kotlin (Part I), and use that as a leverage to use suspendable Kotlin coroutines to write maintainable Non-blocking asynchronous code (Part II). The server application that we develope in these blog posts is a toy server which changes the letter case of English alphabets written to its socket channel (using a terminal application like telnet). for e.g. it would convert `wOw` to `WoW`.


## Setting up the server
Normally we would start bootstrapping our Asynchronous and Non-Blocking server by setting up [`AsynchronousChannelGroup`](https://docs.oracle.com/javase/7/docs/api/java/nio/channels/AsynchronousChannelGroup.html) backed by a customized `ThreadPool`
```kotlin
//...
    @JvmStatic
    fun main(args: Array<String>) {
        val backingThreadPool = Executors.newFixedThreadPool(2)
        val asyncChannelGroup = AsynchronousChannelGroup.withThreadPool(backingThreadPool)
        val address = InetSocketAddress(HOST, PORT)
        val asyncServerSocketChannel = listen(address, asyncChannelGroup)
        // ...
    }
//...
```

_If you are interested for a quick overview on `AsynchronousChannelGroup` here is a short [video](https://youtu.be/SYaWvMHkWS0?si=1jIlizZFUsZrIim7) in which I have explained what purpose it serves._

The `listen` method setups  an `AsynchronousServerSocketChannel` when provided with an `InetSocketAddress` and a `AsynchronousChannelGroup`:

```kotlin
private fun listen(address: InetSocketAddress, group: AsynchronousChannelGroup) = runCatching {
    AsynchronousServerSocketChannel.open(group)
            .bind(address)
            .setOption(StandardSocketOptions.SO_RCVBUF, SOCKET_RECIEVE_BUFFER_SIZE)
    }.onSuccess {
        logger.info("successfully bound to address ${address.hostName}:${address.port}")
    }.onFailure { exception ->
        logger.error("failed to bind to address ${address.hostName}:${address.port}", exception)
}.getOrThrow()

```

As you saw, pretty basic stuff: binding to the provided address for the server to listen for incoming connections and setting the socket recieve buffer size ([more](https://docs.oracle.com/javase/7/docs/api/java/nio/channels/AsynchronousSocketChannel.html) on socket options, please check with your appropriate jdk version doc.)

### Accepting Incoming Connections
To start with accepting incoming connection we need to setup two things:  
1. An Attachment
2. A Callback implementing Completion Handlers

#### Attachment
An `Attachment` can be considered a class that contains arbitary set  of metadata, resources or values _associated_ with an I/O operation. As an example, when we read from or write to an `AsynchronousSocketChannel` the attachment can contain the state of the protocol specific to that socket channel(e.g. number of bytes read or written, Mutexes, Read Write locks). In another words, an Attachment is a way to maintan state across I/O Callback invokations.

Let's design an Attachment class for accepting incoming connections! What we want is to continuously accept new client connections. There is not much state that we need to maintain while we accept new incoming connections. So, it should suffice to only hold on to our precious `AsynchronousServerSocketChannel`:

```kotlin
class ServerSocketChannelAttachment(
    val asynchronousServerSocketChannel: AsynchronousServerSocketChannel
)
```

Now, we need to use this attachment with a Callback that implements `CompletionHandler` to accept incoming connections.

#### `CompletionHandler`

A `CompletionHandler` is the key interface to implement Callbacks which contain the logic to handle when certain I/O event complete.

A `CompletionHandler` interface, contains two important methods: 
* `completed`: is executed when the I/O operation completes successuflly (e.g. a connection is accepted, a read from the underlying channel has finished)
* `failed`: when the I/O operation faces an exception (e.g. read / write fails as client closes connection)

_I have made another [`video`](https://www.youtube.com/watch?v=qquvIgVSBTM) explaining `CompletionHandler` and `Attachment` together._

To register a `CompletionHandler` with type of an I/O operation (e.g. accepting, reading or writing), we need to use our `ServerSocketChannelAttachment` class. Given all that, we are finally ready to implement a `CompletionHandler` Callback that when invoked:

1. Immediately registers another callback to accept other incoming connections
2. Intercepts the incoming client connection represented by `AsynchronousSocketChannel` class
3. Registers a callback for Read I/O event on the client connection (`AsynchronousSocketChannel::read`)

```kotlin
private object AcceptanceCompletionHandler :
    CompletionHandler<AsynchronousSocketChannel, ServerSocketChannelAttachment> {
    override fun completed(
        clientSocketChannel: AsynchronousSocketChannel,
        asyncServerSocketAttachment: ServerSocketChannelAttachment
    ) {
        logger.info("got a connection from ${clientSocketChannel.remoteAddress}")
        // immediately register a callback for accepting new connections.
        asyncServerSocketAttachment.acceptConnection()
        // create a new ClientSocketChannelAttachment class and associate it with this client connection (more on this in the next sections).
        ClientSocketChannelAttachment(clientAsyncChannel = clientSocketChannel).read()
    }

    override fun failed(exc: Throwable, attachment: ServerSocketChannelAttachment) {
        logger.error("accept handler failed with exception", exc)
    }
}
```

Note that we have defined `ServerSocketChannelAttachment.acceptConnection` extension method in the following way:

```kotlin
private fun ServerSocketChannelAttachment.acceptConnection() = runCatching {
    // this is how we register the accept Callback, using the `accept` method on the`AsynchronousServerSocketChannel` that is contained in the attachment class.
    asynchronousServerSocketChannel.accept(
        this, // this refers to current `ServerSocketChannelAttachment` instance 
        AcceptanceCompletionHandler
    )
}.onFailure { exception ->
    logger.error("failed to register accept call back handler on server socket channel", exception)
}.getOrThrow()
```

#### Putting pieces together in the `main` method
Now that we have seen how we can setup our server and register a callback for accepting incoming connection, we can complete our `main` method:

```kotlin
fun main(args: Array<String>) {
    val backingThreadPool = Executors.newFixedThreadPool(2)
    val asyncChannelGroup = AsynchronousChannelGroup.withThreadPool(backingThreadPool)
    val address = InetSocketAddress(HOST, PORT)
    val asyncServerSocketChannel = listen(address, asyncChannelGroup)
    asyncServerSocketChannel.use {
        ServerSocketChannelAttachment(it).acceptConnection()
        joinCurrentThread()
    }
}

private fun listen(address: InetSocketAddress, group: AsynchronousChannelGroup) = {...}

class ServerSocketChannelAttachment(
    val asynchronousServerSocketChannel: AsynchronousServerSocketChannel
)

private fun ServerSocketChannelAttachment.acceptConnection() = runCatching {
    asynchronousServerSocketChannel.accept(
        this,
        AcceptanceCompletionHandler
    )
}.onFailure { exception ->
    //...
}.getOrThrow()

private object AcceptanceCompletionHandler :
    CompletionHandler<AsynchronousSocketChannel, ServerSocketChannelAttachment> {...}

private fun joinCurrentThread() = runCatching {
    Thread.currentThread().join()
}.onFailure { exception: Throwable ->
    logger.error("main thread interrupted with exception", exception)
}.getOrThrow()

```

Here are couple of noteworthy points:

* `CompletionHandler` for accepting incoming connections can be a singletone and hence we used the `object` keyword from Kotlin when implementing `AcceptanceCompletionHandler`.

* We basically can re-use an instance of `ServerSocketChannelAttachment` for registering further callbacks, since all the state it contains is a refrence to the  `AsynchronousServerSocketChannel` which was created in the `main` method.

* Since `AsynchronousServerSocketChannel` implements `Closable` we can take advantage of Kotlin's `use` extension method defined on `Closable` resource in the `main` method body, for clean-up purpose.

* `joinCurrentThread` is called from `main` method to join on the current thread to keep the application running while we are busy handling I/O events.


## Reading and Writing with `AsynchronousSocketChannels`

In earlier section, we immedialetly registered a Callback for reading from incoming client socket connection. In this section we will show how we can achieve that by using a different type of Attachment which we will call `ClientSocketChannelAttachment` and using a `CompletionHandler` implementation named `ReadWriteCompletionHandler` which we will share for both reading and writing.

### Attachment for `AsynchronousSocketChannel`
You should have already noticed the `completed` method signature from `AcceptanceCompletionHandler`:

```kotlin
private object AcceptanceCompletionHandler :
    CompletionHandler<AsynchronousSocketChannel, ServerSocketChannelAttachment> {
    override fun completed(
        clientSocketChannel: AsynchronousSocketChannel // <-- this one,
        asyncServerSocketAttachment: ServerSocketChannelAttachment
    ) {...}
```

when this method is called, we are receiving an instance of `AsynchronousSocketChannel` or what we otherwise would recognize as client connection. This is actually the real deal, the actual asynchronous channel that we will do our buffered reads and writes. Now we are facing with the question of what can we keep inside this attachment `ClientSocketChannelAttachment` for each individual client connection?

Taking a step back, what we want to do is simply to read into a buffer the English letters that a client would write to the socket on their end, change the letter casing (e.g. `a` to `A` or `B` to `b`) and then write that back to the channel using the very same buffer. This gives us following idea what to hold in each Attachment:

1. The `AsynchronousSocketChannel` instance which represents the client connection.
2. A `ByteBuffer` to read into, modify and write from.
3. A boolean flag to indicate the state of the client connection: Either we are Reading into the buffer (read mode) or Writing from the buffer (write mode).


```kotlin
private class ClientSocketChannelAttachment(
    val clientAsyncChannel: AsynchronousSocketChannel,
    var inReadMode: Boolean = true,
    val buffer: ByteBuffer = ByteBuffer.allocate(1024)
)
```

As mentioned earlier, the `clientAsyncChannel` (of type `AsynchronousSocketChannel`) represents the client asynchronous channel that we intercepted from the `AsynchronousServerSocketChannel`. To read or write to the client connection in an asynchronous and non-blocking way, we need to register read and write callbacks, thanks to Callback based APIs provided by the `AsynchronousSocketChannel`. Let's proceed to introduce two extension methods on `ClientSocketChannelAttachment` which register those callbacks re-using the same `ClientSocketChannelAttachment`attachment instance.

```kotlin
private fun ClientSocketChannelAttachment.read() = runCatching {
    clientAsyncChannel.read(
        buffer,
        this,
        ReadWriteCompletionHandler
    )
}.onFailure { exception ->
    logger.error("failed to register read completion handler on client socket channel", exception)
}.getOrThrow()

private fun ClientSocketChannelAttachment.write() = runCatching {
    clientAsyncChannel.write(
        buffer,
        this,
        ReadWriteCompletionHandler
    )
}.onFailure { exception ->
    logger.error("failed to register write completion handler on client socket channel", exception)
}.getOrThrow()
```

We will get into defining `ReadWriteCompletionHandler` in a moment, but what is important from the above extension methods is that we re-use the `ByteBuffer` for read and write. This `ByteBuffer` is contained within the `ClientSocketChannelAttachment` and we will create one instance of the very same attachment per client connection.


### `ReadWriteCompletionHandler`

We are now closing the loop for having a basic async non-blocking server up and running by implementing a `CompletionHandler` which we would re-use for both Read and Write operations.

#### A Single `CompletionHandler` for Reading and Writing 
Just a quick look at callback based API for methods `read` and `write` on `AsynchronousSocketChannel` class, we would understand that we need to implement our `ReadWriteCompletionHandler` in the following way:

```kotlin
private object ReadWriteCompletionHandler : CompletionHandler<Int, ClientSocketChannelAttachment> {
    override fun completed(result: Int, attachment: ClientSocketChannelAttachment) {...}

    override fun failed(exc: Throwable, attachment: ClientSocketChannelAttachment) {...}
}
```

I think the important note from above code is that, `result: Int` parameter tells us how many bytes have been read or written from the buffer that is passe into respective `read` and `write` methods. Here is a quick refresher how we would use a buffer and the `ReadWriteCompletionHandler`:

```kotlin
clientAsyncChannel.read(
    buffer,
    this,
    ReadWriteCompletionHandler
)
```
```
clientAsyncChannel.write(
    buffer,
    this,
    ReadWriteCompletionHandler
)
```

_Just a side note: in an ideal world we should consider having seperate handlers for Reading and Writing._

#### `ReadWriteCompletionHandler` Logic

Within the handler, we need to handle couple of cases:
* if `result` is -1 this means that we no longer can perform read/write on the client socket channel.
* if we read something in the reading mode into the buffer, we will process that buffer content immediately and switch to write mode.
* only after the whole buffer content after modification (in write mode) is written to the client socket channel we will switch to read mode.

The above straight-forward and simple lines of logic will help us to implement a very basic `CompletionHandler` that we can be re-use for both reading and writing to client channels.


```kotlin
    private object ReadWriteCompletionHandler : CompletionHandler<Int, ClientSocketChannelAttachment> {
        override fun completed(result: Int, attachment: ClientSocketChannelAttachment) {
            if (result == -1) {
                logger.info("client at  ${attachment.clientAsyncChannel.remoteAddress} disconnected")
                attachment.close()
                return
            }

            if (attachment.inReadMode) {
                // if reading 0 bytes, just register read callback.
                if (result == 0) {
                    attachment.inReadMode = true
                    attachment.read()
                    return
                }

                // in a different case, we now process the data in the buffer.
                attachment.inReadMode = false
                attachment.buffer.flip()
                Utils.changeLetterCase(attachment.buffer)
                attachment.write()
            } else {
                // while not in read mode, since there is data in the buffer, we keep writing
                // until the buffer is empty.
                if (attachment.buffer.hasRemaining()) {
                    attachment.write()
                } else {
                    // since there is no data remaining in the buffer, we are now, ready to switch to read mode.
                    attachment.inReadMode = true
                    attachment.buffer.clear()
                    attachment.read()
                }
            }
        }

        override fun failed(exc: Throwable, attachment: ClientSocketChannelAttachment) {
            val op = if (attachment.inReadMode) "read" else "write"
            logger.error("$op failed for client: ${attachment.clientAsyncChannel.remoteAddress}", exc)
            attachment.close()
        }

        private fun ClientSocketChannelAttachment.close() = runCatching {
            clientAsyncChannel.close()
        }.onFailure { exception ->
            logger.error("IO exception while closing client connection", exception)
        }

    }

}

```

Also here is the code which modifies the `ByteBuffer` content based on their Letter casing:

```kotlin
object Utils {
     private const val SPACE_BYTE = ' '.code.toByte()
     fun changeLetterCase(buffer: ByteBuffer) {
        for (pos in 0..<buffer.limit()) {
            val char = buffer.get(pos).toInt().toChar()
            if (char.isLetter()) {
                buffer.put(
                    pos,
                    char.code.toByte().xor(SPACE_BYTE)
                )
            }
        }
}
```

## Testing the server

You can use `telnet` to test the [server](https://github.com/rasouli/blog-codes/blob/main/jvm/kotlin-java-nio/kotlin-java-nio/src/main/kotlin/rs/reza/pub/nioserver/BasicAsyncServer.kt) we wrote in this part on the command line:

```
 telnet localhost 6070
```

## Wrapping Up Part I

The whole application code can be found [here](https://github.com/rasouli/blog-codes/tree/main/jvm/kotlin-java-nio/kotlin-java-nio). In Part I, we learned how we can create a Non-blocking asynchronous server using Java NIO 2 and Kotlin using its Callback based API. In Part II (stay tuned!) we will continue to take advantage of Kotlin Coroutines to write more maintainable code while achieving the same functionality in [Part II](blog/b-2025-07-27-kt-java-nio2-async-channels-p2.md).
