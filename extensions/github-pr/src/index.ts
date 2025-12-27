/**
 * GitHub PR Integration Extension
 *
 * Provides commands to list and manage GitHub Pull Requests.
 */

import { defineExtension } from "@opencmd/extension-api";

interface PullRequest {
  title: string;
  number: number;
  url: string;
  author: string;
  state: string;
  draft: boolean;
}

interface GitHubConfig {
  token: string;
  default_repo?: string;
}

defineExtension({
  commands: [
    {
      id: "list-prs",
      async handler(ctx, input) {
        const config = await ctx.getConfig<GitHubConfig>();
        const repo = input || config.default_repo;

        if (!repo) {
          await ctx.showToast(
            "Please specify a repository or set a default",
            "error"
          );
          return;
        }

        const response = await ctx.fetch(
          `https://api.github.com/repos/${repo}/pulls?state=open`,
          {
            headers: {
              Authorization: `token ${config.token}`,
              Accept: "application/vnd.github.v3+json",
            },
          }
        );

        if (!response.ok) {
          await ctx.showToast(`Failed to fetch PRs: ${response.statusText}`, "error");
          return;
        }

        const prs: PullRequest[] = await response.json();

        await ctx.showResults(
          prs.map((pr) => ({
            title: `#${pr.number}: ${pr.title}`,
            subtitle: `by ${pr.author} ${pr.draft ? "(Draft)" : ""}`,
            data: pr,
          }))
        );
      },
    },
    {
      id: "my-prs",
      async handler(ctx) {
        const config = await ctx.getConfig<GitHubConfig>();

        const response = await ctx.fetch(
          "https://api.github.com/search/issues?q=is:pr+is:open+author:@me",
          {
            headers: {
              Authorization: `token ${config.token}`,
              Accept: "application/vnd.github.v3+json",
            },
          }
        );

        if (!response.ok) {
          await ctx.showToast(`Failed to fetch PRs: ${response.statusText}`, "error");
          return;
        }

        const data = await response.json();
        const prs = data.items || [];

        await ctx.showResults(
          prs.map((pr: PullRequest) => ({
            title: pr.title,
            subtitle: pr.url,
            data: pr,
          }))
        );
      },
    },
    {
      id: "review-prs",
      async handler(ctx) {
        const config = await ctx.getConfig<GitHubConfig>();

        const response = await ctx.fetch(
          "https://api.github.com/search/issues?q=is:pr+is:open+review-requested:@me",
          {
            headers: {
              Authorization: `token ${config.token}`,
              Accept: "application/vnd.github.v3+json",
            },
          }
        );

        if (!response.ok) {
          await ctx.showToast(`Failed to fetch PRs: ${response.statusText}`, "error");
          return;
        }

        const data = await response.json();
        const prs = data.items || [];

        await ctx.showResults(
          prs.map((pr: PullRequest) => ({
            title: pr.title,
            subtitle: `Review requested`,
            data: pr,
          }))
        );
      },
    },
  ],

  async onLoad(ctx) {
    const config = await ctx.getConfig<GitHubConfig>();
    if (!config.token) {
      await ctx.showToast("GitHub token not configured", "error");
    }
  },
});
