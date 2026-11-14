import rss from "@astrojs/rss";
import { getCollection } from "astro:content";
import { getUniqueTags } from "@/utils/getUniqueTags";
import { getSortedPosts } from "@/utils/getSortedPosts";
import { getPostUrl } from "@/utils/getPostPaths";
import { slugifyAll } from "@/utils/slugify";
import config from "@/config";

// Per-tag feeds kept at the paths the Zola site served them from, so existing
// subscriptions keep working. The content is RSS; feed readers sniff the format.
export async function getStaticPaths() {
  const posts = await getCollection("posts", ({ data }) => !data.draft);

  return getUniqueTags(posts).map(({ tag, tagName }) => ({
    params: { tag },
    props: {
      tagName,
      posts: getSortedPosts(
        posts.filter(({ data }) => slugifyAll(data.tags).includes(tag))
      ),
    },
  }));
}

export async function GET({
  props,
}: {
  props: {
    tagName: string;
    posts: Awaited<ReturnType<typeof getCollection<"posts">>>;
  };
}) {
  return rss({
    title: `${config.site.title} - ${props.tagName}`,
    description: config.site.description,
    site: config.site.url,
    items: props.posts.map(({ data, id, filePath }) => ({
      link: getPostUrl(id, filePath, config.site.lang),
      title: data.title,
      description: data.description,
      pubDate: new Date(data.modDatetime ?? data.pubDatetime),
    })),
  });
}
