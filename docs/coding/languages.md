# Languages, Careers and the Long Game

I have learnt [many programming languages](https://github.com/rkfcheung/coding) over the years. Some I barely touched beyond tutorials. Some faded out of the market. A much smaller group became the tools I use fluently in real work.

Today, the ones I genuinely master are Java, Kotlin, Python and Rust. I can live-code in them, write idiomatic code, and I have real professional experience using them. Plenty of others sit in the background, sometimes useful, sometimes nothing more than past history.

This is not a ranking of languages or a technical comparison. It is simply how they have appeared in my career: where they are used, how marketable they have been, and how I think about them now. Your path may look very different.

## Java, Kotlin and the JVM Crowd

**Java** has been the main thread running through my career. It still underpins a huge amount of enterprise software. Banking, trading, insurance, government systems, large retailers and just about every established company have Java somewhere in their stack. If you want a stable route into backend engineering, Java remains one of the safest choices.

Most Java roles I have seen cluster around a few areas:

* Enterprise backends such as REST APIs and microservices
* Financial systems such as risk engines and trading platforms
* Modernisation of larger, older systems

The upside is obvious: abundant jobs, mature tooling and decades of experience across the industry. The downside is that many codebases are large, old and politically slow to evolve. But if you can navigate complexity with patience, Java creates opportunities.

**Kotlin** arrived later for me but fitted in effortlessly. If you already know Java, Kotlin feels like a calmer, tidier version of it. The type system is more expressive, the syntax is more concise and the language supports modern patterns without leaving the JVM ecosystem.

I have mostly used Kotlin next to Java, adding it piece by piece into existing systems. In the wider industry, Kotlin is best known for Android development, where it is now the default language.

Other JVM languages such as **Groovy**, **Scala** and **Clojure** are more niche from my perspective. Groovy mostly appears in build pipelines and automation. Scala had a strong wave of popularity, especially in data platforms and some financial systems, and remains entrenched in certain organisations. Clojure has a loyal following among functional programmers, though I have not used it in production.

If you want marketability, Java or Kotlin is a reliable path.

## Python

For me, **Python** has mainly been a backend language: web APIs, internal tooling and small services that sit between larger systems. Across the industry, though, Python is everywhere.

* Data science and analytics
* Machine learning and AI
* Automation and DevOps tooling

Its biggest advantage is speed of iteration. You can get from idea to working prototype in very little time. That is why it dominates notebooks, experiments and exploratory work.

From a career perspective, Python is a strong choice. It is a natural route into data and ML, and it also serves as a common foundation for a large amount of cloud tooling.

In terms of opportunity, Python is hard to beat. It opens doors in many different directions.

## Rust

**Rust** has been the hardest language I have [learnt](learn-rust.md), and one of the most rewarding. My first attempts were slow and frustrating. Concepts like ownership, borrowing and lifetimes took time before they clicked. But after getting past the initial wall, Rust shifted my career in a new direction.

I moved from typical JVM and Python backend work into more systems-oriented areas, particularly [blockchain protocols](blockchain-journey.md) and reliability-focused components. This shift did not happen quickly. It grew gradually through side projects, exercises and a lot of reading.

Rust is niche compared to Java or Python, but the niches are very interesting.

* Blockchains and cryptography-heavy systems
* Databases and storage engines
* High-performance services with strict reliability requirements
* Developer tooling and infrastructure

Expectations can be high. Rust roles often sit close to systems programming, and comparisons with C and C++ developers are not unusual. But if you enjoy thinking about safety, correctness and performance together, Rust is a strong long-term choice.

It is also the language where [defensive coding](defensive-coding.md) feels most natural. You are encouraged to make invariants explicit, avoid "this cannot fail" assumptions and treat panics as design issues.

## C#

**C#** fills a similar space to Java but sits closer to the Microsoft ecosystem. It is widely used for enterprise systems, especially in companies standardised on Windows and .NET. It also shows up in desktop tools, internal line-of-business applications and game development through Unity.

C# is cross-platform now, but much of its demand still comes from organisations where Windows is the default environment.

If you know Java, C# is easy to learn, and the reverse is true as well. The real question is whether you want to work in the .NET ecosystem. If you do, C# is powerful and very marketable. If not, it is still a useful adjacent language rather than a primary one.

## JavaScript and TypeScript

I worked as full-stack for a period before moving more firmly towards backend development. During that time, **JavaScript** and **TypeScript** were unavoidable.

JavaScript runs everywhere, but TypeScript has become the standard for larger, more maintainable projects. The static typing and clearer structure make a noticeable difference once a codebase grows.

In practice, mastering the language is only part of the picture. Frameworks and libraries matter more, because the front-end ecosystem moves quickly. React, Vue, Node.js and countless build tools define the day-to-day experience as much as the language itself.

Together, JavaScript and TypeScript cover everything from front-end interfaces to server-side APIs. If you want true full-stack versatility, they remain central.

## C and C++

**C** was my first professional language after graduating. I have not used it heavily since, but understanding it properly still pays off. It gives you a concrete mental model of memory, pointers and undefined behaviour, and it helps you understand what higher-level languages are doing behind the scenes.

Modern **C++** is very different from the older style I used early on. Features such as smart pointers, move semantics, concepts and ranges make it feel like a new language.

C++ roles tend to appear in a few domains:

* Performance-critical systems such as trading platforms, real-time engines and rendering
* Game development and game engines
* Legacy but important systems that cannot easily be rewritten

The learning curve is steep. If you are already comfortable with Rust, modern C++ can feel strangely familiar and unfamiliar at the same time. For me, [revisiting](relearn-cpp.md) it has been more curiosity than necessity, but it sharpens low-level thinking.

## Other Languages

### Go

Even though I do not write **Go** professionally, it is hard to ignore. Many modern infrastructure tools are written in Go: container tooling, cloud-native systems, networking components and observability stacks.

If you enjoy backend engineering with a focus on reliability and simplicity, Go can be a strong option.

### Swift

**Swift** is the main language for iOS and macOS development. I originally worked with Objective-C before moving to Swift for app development. While server-side Swift exists, the majority of demand remains in mobile and desktop applications on Apple platforms.

If you enjoy user interfaces and product-focused work, Swift offers a very direct route.

### Solidity

**Solidity** is the most common language for writing smart contracts on Ethereum. There are many online bootcamps and courses for it. The ecosystem is still evolving rapidly, but knowledge of Solidity is useful if you want to explore blockchain protocols, smart contract development or decentralised applications.

### Lua

**Lua** appears in unexpected places. It is often embedded inside game engines and tools, and it works well for scripting and configuration. When I used Lua, it felt close to Python: dynamic, flexible and easy to embed. It is rarely a primary job requirement but useful in specific domains.

## Pick a Life, Then Pick a Language

People often start by asking which language they should learn. That is not a bad question, but I have found another one more helpful:

What kind of problems do I want to work on, and what sort of life do I want around that work?

Once you answer that, the language choices fall into place.

* For stable enterprise work, Java, C# or C++ makes sense.
* For data and ML, Python is the natural starting point.
* For systems programming and performance, C++ or Rust fit well.
* For web development, JavaScript and TypeScript underpin almost everything.
* For infrastructure and cloud tooling, Go and Python are difficult to avoid.
* For mobile ecosystems, Swift for iOS or Kotlin for Android.
* For blockchain work on EVM, Solidity is unavoidable.

Interviews and careers can still be messy and unpredictable. I have had my share of [failed interviews](prep-interviews.md) and [moments of doubt](unlucky-unhappy.md). But looking back, the languages that stayed with me were the ones connected to real work and real curiosity, not just tutorial enthusiasm.

You can regret not learning something earlier, or you can decide what kind of engineer you want to become next and choose the languages that support that direction.

The tools matter. The direction matters more.
