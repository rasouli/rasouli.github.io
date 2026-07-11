---
title: "Migrating From Bearblog to Properdocs"
slug: b-2026-07-11-properdocs
date: 2026-07-11 17:00:00
publish: true
tags:
  - blog
  - properdocs
---

![](assets/bye-bear.jpg)
> Photo by [Hans-Jurgen Mager](https://unsplash.com/@hansjurgen007?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText) on [Unsplash](https://unsplash.com/photos/polar-bear-on-snow-covered-ground-during-daytime-qQWV91TTBrE?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText)
      
Some time ago I decided to migrate my blog off of [Bearblog](bearblog.dev) mainly because I had to pay a subscription fee for attaching my own domain, which I did not like very much. Don't get me wrong, Bearblog provides a great service, but I thought "it is the time" this weekend. In this post I explain how `properdocs` can help you with that to host your personal blog on top of github pages.

<!-- more -->

## Markdown All The Way
Looking for a Markdown centric blogging alternative, I first stumbled upon [Mkdocs](https://github.com/mkdocs/mkdocs) which was very promising. It looked simple and straight forward. Unfortunately it seems the upcoming version of Mkdocs would introduce some breaking changes that would break backward compatibility with some plugins (afaik) so thankfully a fork exists called [Properdocs](https://github.com/ProperDocs/properdocs). I decided to enjoy the existing [catalog](https://github.com/mkdocs/catalog) of plugins by using only Properdocs to build up my blog.

## Raw Properdocs -> Blog
As I soon learned, getting started with raw Properdocs does not necessarily mean that you will end up with a full featured blog, so that's where plugins come in place.
For that I found [mkdocs-publisher](https://github.com/mkdocs-publisher/mkdocs-publisher) very handy. This plugin (well more like bundle of plugins) gives you following capabilities:
- [`pub-meta`](https://mkdocs-publisher.github.io/setup/general/pub-meta/) enable you to add metadata to your markdown blog post (things like slug, publish flag, date and title)
- [`pub-blog`](https://mkdocs-publisher.github.io/setup/general/pub-blog/) gives you goodies like building archives, tags and categories

Note:
> There are more plugins in mkdocs-publisher which I highly encourage you to look at, they even have `pub-obsidian`!  
>  

Note: 
>Also `pub-blog` comes with Material theme out of the box which is Exxxxtraaa nice.


## URL Redirect
Another problem I had with migrating my blog off of Bearblog was making my Properdocs blog to map certain URLs to their new counterparts. After all, I had shared some links to my previous blog posts over Social Media and I wanted to make sure old links work and redirect properly. That's when the [mkdocs-redirect](https://github.com/mkdocs/mkdocs-redirects) saved me. You can simply map your old Bearblog post urls to their new url in the following way:

```yaml
# properdocs.yml
plugins:
   - redirects:
       redirect_maps:
         'b-2025-05-24-kt-java-nio2-async-channels-p1.md' : 'blog/b-2025-05-24-kt-java-nio2-async-channels-p1.md'
```

Note:
> The trick was in adding the `.md` at the end of old post url to make redirect work.

## Working Around Annoying NavBar
To get fine grained control over what shows up in your Navbar and what should be hidden,  [`mkdocs-awesome-pages-plugin`](https://pypi.org/project/mkdocs-awesome-pages-plugin/) saved me. It gives you lots of flexibility how to customize your Navbar, So check it out!

## Handling CNAME
I deploy using a branch from github pages, and one annoying thing with static website generators is to always find a way to copy the CNAME file to your deployment branch (or copy that alongside your statically generated web site artifacts). You can do this with properdocs by following configuration:

```yaml
#properdocs.yml
extra_templates:
  - CNAME
```

Note:
> CNAME needs to live inside the `docs` directory.

## Highlighting Code
Yes, I need some color on my code snippets. That's where you need [`pymdownx`](https://facelessuser.github.io/pymdown-extensions/) which is pulled in by default by [`mkdocs-material`](https://squidfunk.github.io/mkdocs-material/) (Thanks to `pub-blog` plugin). Here is the configuration that you need for making the highlight to work:

```yaml
#properdocs.yaml
theme:
  name: material

markdown_extensions:
  - pymdownx.highlight:
       auto_title: false
       anchor_linenums: true
       line_spans: __span
       pygments_lang_class: true
       use_pygments: true
  - pymdownx.snippets
  - pymdownx.superfences
```
Note:
> Good news, there are far more goodies to [configure](https://squidfunk.github.io/mkdocs-material/setup/).

## Workflow
My current workflow now is the following for writing a new blog post:
```markdown
1. Pick an idea
2. Create an `.md` file in the `blog/` directory
3. fire up `properdocs serve` in the background to continuously see live updates in browser as I type in my editor.
4. Review the post
5. use `properdocs build` to make sure static website builds correctly.
6. commit changes in main branch if everything looks good.
7. use `properdocs gh-deploy` to ship changes.
```


## A Thank You
I need to end this post with big thank you to all awesome people who make `Mkdocs`, maintain `Properdocs` and create all these amazing plugins! THANK YOU!
