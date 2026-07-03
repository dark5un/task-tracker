The Philosophical Developer
Chapter 6

The pipeline on my own machine.

The whole point of this series is local inference for development. Data ownership matters. When the model runs on your machine, the code never leaves. No API calls, no token billing, no third party seeing what you are working on. I want development to work on my own machine. The cloud is just a tool to move faster while we figure things out.

DeepSeek V4 Flash is the reason this is possible. I use DeepSeek's cloud service and I am really happy with it. It is insanely cheap and does the job I need. Full credit where it is due.

After establishing the methodology on cloud infrastructure, I needed to know one thing. Does the pipeline depend on the cloud, or does it work anywhere? I tested it on my own hardware. The results exceeded expectations. The pipeline works on a laptop just as well as anywhere else.

What I built is not a product. It is four git commands and a naming convention. It is an experiment shared as open source. The question I am exploring is simple: how little infrastructure do you need to make AI-generated code auditable? The answer so far is: git, a tag naming convention, and the willingness to write tests first.

The pipeline is not tied to the cloud. It is not tied to any model. It is a process, and processes travel where they are needed.

DeepSeek V4 Flash made this possible. It runs on my laptop at 14 watts. That is the future I want to build toward.