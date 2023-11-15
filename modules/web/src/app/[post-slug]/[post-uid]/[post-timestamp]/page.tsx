import React from 'react';
import {gql, GraphQLClient} from "graphql-request";
import Markdown from "markdown-to-jsx";
import {notFound, redirect} from "next/navigation";
import {Metadata} from "next";

export async function generateMetadata(
  {params}: any
): Promise<Metadata> {
  const data: any = await fetchData(params["post-uid"], params["post-timestamp"]);
  if (data.post == null) {
    return {}
  }
  const {uid, timestamp, filePath, contentMarkdown} = data.post
  const title = contentMarkdown.match(/title: (.+)/)[1];
  const slug = filePath.replace(".md", "");
  if (params["post-slug"] != slug) {
    redirect(`/${slug}/${uid}/${timestamp}`);
  }
  const tags = (contentMarkdown as string).match(/---(?:.|\n)+tags:((.|\n)+)---/)?.[1]?.replaceAll(/[\n-]/g, "").split(" ").map(x => x.trim()).filter(x => x);
  return {
    title: title,
    metadataBase: new URL('https://blog.railson.dev'),
    openGraph: {
      type: 'article',
      title: title,
      url: `https://blog.railson.dev/${slug}/${uid}/${timestamp}`,
      images: ['/seo.png'],
      publishedTime: new Date(Number(uid) * 1000).toISOString(),
      modifiedTime: new Date(Number(timestamp) * 1000).toISOString(),
      authors: [
        'https://railson.dev'
      ],
      tags: tags,
    },
  }
}

export default async function Page({params}: any) {
  // TODO: verify cache
  const data: any = await fetchData(params["post-uid"], params["post-timestamp"]);
  if (data.post == null) {
    notFound()
  }
  const {contentMarkdown, timestamp, gitRevision, filePath} = data.post

  const title = contentMarkdown.match(/title: (.+)/)[1];

  const tags = (contentMarkdown as string).match(/---(?:.|\n)+tags:((.|\n)+)---/)?.[1]?.replaceAll(/[\n-]/g, "").split(" ").map(x => x.trim()).filter(x => x);

  return (
    <>
      <main className="flex flex-col items-center justify-between p-24" >
        <h1>
          {title}
        </h1>
        <span>
          {new Date(timestamp*1000).toUTCString()}
        </span>
        <br/>
        <Markdown>{contentMarkdown.replace(/---(.|\n)+---/g, "")}</Markdown>
        <br/>

        <ul>
          {tags?.map((tag, index) => {
            return <li className="inline">
              {index>0 ? " - ": ""}{tag}{" "}
            </li>
          })}
        </ul>
        <a className={"text-[#8c74d2]"} href={`https://github.com/railson-ferreira/railson-dev-blog/blob/${gitRevision}/posts/${filePath}`}>
          View on Github
        </a>
      </main>
    </>
  );
}

async function fetchData(postUid: string, postTimestamp: string) {
  const query = gql`{
      post(uid: "${postUid}",updatingDateTime: "${new Date(Number(postTimestamp) * 1000).toISOString().replace(".000Z", "Z")}") {
        uid,
        gitRevision,
        timestamp,
        filePath,
        contentMarkdown
      }
    }`
  const client = new GraphQLClient(process.env.GRAPHQL_URL!)
  return await client.request(query)
}

export const revalidate = 60 * 60 // 1 hour