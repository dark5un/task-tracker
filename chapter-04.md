The Philosophical Developer
Chapter 4

Making thought visible.

After the first experiment I had a new problem. The LSP tether worked. The AI wrote good code. Tests passed, coverage was high. But when I looked at the repository, I saw one commit. One wall of code. No trace of how we got there.

An AI does not explain itself. It produces. And production without explanation is hard to trust, especially when you have to review it later.

I wanted to see the chain of thought. Not in a chat log. In the code itself. I wanted to open a repo and see: here is where the test was written first. Here is where the implementation followed. Here is where things got cleaned up.

The solution was something every developer already has. Git.

The idea was simple. Each feature gets its own branch. Each TDD cycle on that branch gets its own commit. When the feature is done, squash everything into main for a clean history. But before deleting the branch, tag its tip. That tag preserves the chain.

I tested this with a task tracker CLI. Four features. Eight TDD cycles. Four branches. I pushed everything to GitHub with the tags.

Now when I look at the repo I see two things. Main shows the clean story. "feat: add task creation. feat: list tasks. feat: mark tasks complete. feat: JSON persistence." That is what a human wants to read.

But if I want to see how the AI got there, I run git log trace/add-task and I see the cycles. Cycle 1: Task model. Cycle 2: add subcommand. Each commit is a step in the reasoning.

The tag does not lie. It points at the exact state of the branch when the feature was complete. It survives branch deletion. It can be pushed to remote. It is searchable.

This is still early. The pattern works for sequential work. For parallel agents with worktrees, it is designed but not yet tested. But the foundation is solid.

What I learned is that traceability is not about archives. It is about trust. If I can see the steps, I can verify the thinking. And if I can verify the thinking, I can let the AI write more code with less supervision.

The repo is at dark5un/task-tracker. The tags are there. Go look.
