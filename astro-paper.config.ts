import { defineAstroPaperConfig } from "./src/types/config";

export default defineAstroPaperConfig({
  site: {
    url: "https://damszew.dev/",
    title: "damszew",
    description:
      "Personal blog for sharing thoughts about different practices and patters used to create clean and efficient software",
    author: "Damian Szewczyk",
    profile: "https://github.com/damszew",
    lang: "en",
    timezone: "Europe/Warsaw",
    dir: "ltr",
  },
  posts: {
    perPage: 12,
    perIndex: 4,
  },
  features: {
    lightAndDarkMode: true,
    dynamicOgImage: true,
    showArchives: false,
    showBackButton: true,
    editPost: { enabled: false },
    search: "pagefind",
    toc: true,
  },
  socials: [
    { name: "github", url: "https://github.com/damszew" },
    { name: "linkedin", url: "https://www.linkedin.com/in/szewczyk-damian/" },
    { name: "x", url: "https://twitter.com/damszew/" },
  ],
  shareLinks: [],
});
