The Philosophical Developer
Chapter 5

Polishing the output.

I had three repos full of chapters and articles. Markdown files. Well written, I thought. But I wondered: how clean are they really? Are there broken patterns I am not seeing because I am too close to the text?

So I installed some tools to check.

markdownlint caught two things right away. First, the line length rule. Default is 80 characters, which works for code but not for prose. Every single paragraph in every chapter was flagged. I turned that rule off for chapters. Second, a missing newline at the end of each file. Easy fix.

vale checks for style and spelling. It flagged words like "subcommands" and "worktrees" and "repo". Those are correct technical terms. I left them as is. Vale is useful for finding actual typos, but it needs a custom dictionary for technical writing.

markdown-link-check would catch broken URLs. I did not run it on the chapters since they don't have external links, but it will be useful for READMEs that reference GitHub repos.

I also tried to add a markdown language server so the editor would show diagnostics while writing, the same way rust-analyzer does for Rust code. The server installed but had a compatibility issue with the current Node version. Worth revisiting later.

The exercise taught me something simple. Tools are not a replacement for reading your own work. But they catch the things you stop seeing after the tenth edit. A missing newline. An inconsistent heading style. A line that runs too long.

I added a markdownlint config to all three repos so the settings are shared. Next time I write a chapter, I can run markdownlint before committing and catch the small things automatically.

The Philosophical Developer series now spans five chapters across three repos. Each one is a real experiment with real code, real tests, and real outcomes. The tools help keep the writing as clean as the code.