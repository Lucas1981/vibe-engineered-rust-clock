# Vibe Engineered Rust Clock

So this project serves two purposes: taking some small steps into the realm of learning how to work with Rust better, and taking a more serious approach to Vibe Engineering. I liked the distinction that I heard Ed Donner make first - although he attributed the term at least to Andrej Karpathy - between Vibe Coding and Vibe Engineering, where the Vibe Coding is more a prompt once and watch it run kind of exercise, whereas Vibe Engineering is more planned, organized and you monitor the progress also step by step.

A Rust clock seemed like a good project to try this out with as well. At first I set up the AGENTS.md file, which is why I also kept that one in this repository. I went off the rails in the first two steps multiple times, so I would see where we would go wrong, iterate on the AGENTS.md file to make it more constrained and make the specs more clear, then try again. That works quite well and gets some good results.

Then I'd take things slowly and build things up step by step, iterating over each of the points in the AGENTS.md file. By the time I hit fonts, for example, the LLMs had made up binary strips that together represented the numbers to use in graphical representation. Very cute, but I was more interested in standard practice of using fonts for canvases in Rust, so I decided to once again specify that we'd want to use actual fonts, and then I went back to the drawing board again.

After some iterations, I was finally able to get a nice and decent clock to show. It is very good because at that point you can go back to how things are done and learn more about Rust and how things work. I would sometimes stop to ask some question about why things are the way they are and how things actually work. I would after a while then store the insights about scope and data zipping for instance in a LEARNINGS.md file, then rewind in the conversation to free up some context, and then continue with next steps. That's a great way to manage the context window.

This project is small enough to do in one context window session, there was no real reason to go and compact the context window.
