---
publish: 'true'
search:
  exclude: true
slug: .
title: Blog

---

<!--
  ~ MIT License
  ~
  ~ Copyright (c) 2023-2025 Maciej 'maQ' Kusz <maciej.kusz@gmail.com>
  ~
  ~ Permission is hereby granted, free of charge, to any person obtaining a copy
  ~ of this software and associated documentation files (the "Software"), to deal
  ~ in the Software without restriction, including without limitation the rights
  ~ to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  ~ copies of the Software, and to permit persons to whom the Software is
  ~ furnished to do so, subject to the following conditions:
  ~
  ~ The above copyright notice and this permission notice shall be included in all
  ~ copies or substantial portions of the Software.
  ~
  ~ THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  ~ IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  ~ FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  ~ AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  ~ LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  ~ OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  ~ SOFTWARE.
  -->


## [Introduction to Project Panama (Part I): Loading a Native Library](http://127.0.0.1:8000/blog/b-2026-04-06-itpplnl-p1/)

<!--suppress LongLine -->
<div class="post-extra">
    <div class="col">
        <p class="post-date">2026-04-06 22:46:00</p>
    </div>
    <div class="col">
    
        <a href="http://127.0.0.1:8000/blog/tags/panama/">#panama</a>
    
        <a href="http://127.0.0.1:8000/blog/tags/java/">#java</a>
    
        <a href="http://127.0.0.1:8000/blog/tags/FFM/">#FFM</a>
    
        <a href="http://127.0.0.1:8000/blog/tags/foreign/">#foreign</a>
    
        <a href="http://127.0.0.1:8000/blog/tags/native/">#native</a>
    
    </div>
</div>

![efbffaf3-city-18478-160db8f19ec](assets/efbffaf3-city-18478-160db8f19ec.webp)

Project Panama is Java's modern alternative for calling native code or “Foreign Functions” implemented in other languages such as C or C++. While previously we had Java Native Interface (JNI) for achieving the same goal by writing glue code (C files translating types between Java and C) to connect the dots between the JVM and shared `so` or `dll` libraries, now Project Panama helps us by achieving that all in Java :)  




<div class="post-link">

    <a href="http://127.0.0.1:8000/blog/b-2026-04-06-itpplnl-p1/" title="Introduction to Project Panama (Part I): Loading a Native Library">
        Read more
    </a>

</div>


## [Engine Rooms of Asynchronous Processing (part 1): Reactor Pattern Conceptual Overview](http://127.0.0.1:8000/blog/b-2025-08-30-erap-p1/)

<!--suppress LongLine -->
<div class="post-extra">
    <div class="col">
        <p class="post-date">2025-08-30 13:00:00</p>
    </div>
    <div class="col">
    
        <a href="http://127.0.0.1:8000/blog/tags/asyn/">#asyn</a>
    
        <a href="http://127.0.0.1:8000/blog/tags/nio/">#nio</a>
    
    </div>
</div>

"Engine Rooms of Asynchronous Processing" are series of articles exploring patterns used to facilitate concurrent asynchronous programming. The series aims to (eventually) provide a clear understanding of well-established patterns in this space and explore implementing them from scratch.

The first group of these articles will explore the Reactor pattern.




<div class="post-link">

    <a href="http://127.0.0.1:8000/blog/b-2025-08-30-erap-p1/" title="Engine Rooms of Asynchronous Processing (part 1): Reactor Pattern Conceptual Overview">
        Read more
    </a>

</div>


## [The Social Life of Software: Bounded Contexts Relationships in Action (Part II)](http://127.0.0.1:8000/blog/b-2025-08-24-social-life-software-bc-rel-p2/)

<!--suppress LongLine -->
<div class="post-extra">
    <div class="col">
        <p class="post-date">2025-08-24 18:00:00</p>
    </div>
    <div class="col">
    
        <a href="http://127.0.0.1:8000/blog/tags/ddd/">#ddd</a>
    
    </div>
</div>

This is a follow-up post from [Part I](blog/b-2025-08-16-social-life-software-bc-rel-p1.md) where we looked into Cooperation relationship type between Bounded Contexts. This post will look into following topics:
* Customer-Supplier 
* Separate Ways
* Context Map




<div class="post-link">

    <a href="http://127.0.0.1:8000/blog/b-2025-08-24-social-life-software-bc-rel-p2/" title="The Social Life of Software: Bounded Contexts Relationships in Action (Part II)">
        Read more
    </a>

</div>


## [The Social Life of Software: Bounded Contexts Relationships in Action (Part I)](http://127.0.0.1:8000/blog/b-2025-08-16-social-life-software-bc-rel-p1/)

<!--suppress LongLine -->
<div class="post-extra">
    <div class="col">
        <p class="post-date">2025-08-16 13:05:00</p>
    </div>
    <div class="col">
    
        <a href="http://127.0.0.1:8000/blog/tags/ddd/">#ddd</a>
    
    </div>
</div>

We have all heard about Anti-Corruption Layers, Open-Host, Separate Ways, Conformist etc etc, it just happens that on a day to day basis we are dealing with these abstractions while we might not be very conscious about them. In this post, I try to provide concrete examples of how these Bounded Context relationships are in play.




<div class="post-link">

    <a href="http://127.0.0.1:8000/blog/b-2025-08-16-social-life-software-bc-rel-p1/" title="The Social Life of Software: Bounded Contexts Relationships in Action (Part I)">
        Read more
    </a>

</div>


## [Kotlin Coroutines and Java NIO 2 Asynchronous Channels (Part II)](http://127.0.0.1:8000/blog/b-2025-07-27-kt-java-nio2-async-channels-p2/)

<!--suppress LongLine -->
<div class="post-extra">
    <div class="col">
        <p class="post-date">2025-07-27 00:00:00</p>
    </div>
    <div class="col">
    
        <a href="http://127.0.0.1:8000/blog/tags/java/">#java</a>
    
        <a href="http://127.0.0.1:8000/blog/tags/kotlin/">#kotlin</a>
    
    </div>
</div>

We showed that using Kotlin Coroutines and its ecosystem, we can write much more maintainable code when taking advantage of Java NIO 2 Asynchronous Channels. This blog post is the follow-up and last part of [previous](blog/b-2025-05-24-kt-java-nio2-async-channels-p1.md) blog post where we managed to write a very simple Asynchronous and Non-blocking Java NIO server using [AsynchronousServerSocketChannel](https://docs.oracle.com/javase/8/docs/api/java/nio/channels/AsynchronousServerSocketChannel.html). Now in this part we will leverage Kotlin coroutines and rewrite the whole thing. The end result will be interesting in that we end up with a more maintainable code!




<div class="post-link">

    <a href="http://127.0.0.1:8000/blog/b-2025-07-27-kt-java-nio2-async-channels-p2/" title="Kotlin Coroutines and Java NIO 2 Asynchronous Channels (Part II)">
        Read more
    </a>

</div>

