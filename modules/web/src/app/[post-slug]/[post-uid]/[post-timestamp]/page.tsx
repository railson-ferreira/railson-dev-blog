import React from 'react';
import {gql, GraphQLClient} from "graphql-request";
import Markdown from "markdown-to-jsx";
import {notFound, redirect} from "next/navigation";
import {Metadata} from "next";

export async function generateMetadata(
  { params }: any
): Promise<Metadata> {
  const data: any = await fetchData(params["post-uid"], params["post-timestamp"]);
  if (data.post == null) {
    return {}
  }
  const {uid, timestamp, filePath, contentMarkdown} = data.post
  const title = contentMarkdown.match(/title: (.+)/)[1];
  const slug = filePath.replace(".md", "");
  if(params["post-slug"]!=slug){
    redirect(`/${slug}/${uid}/${timestamp}`);
  }
  return {
    title: title,
    openGraph: {
      type: 'article',
      title: title,
      url: `https://blog.railson.dev/${slug}/${uid}/${timestamp}`,
      images: ['/seo.png'],
      publishedTime: new Date(Number(uid)*1000).toISOString(),
      modifiedTime: new Date(Number(timestamp)*1000).toISOString(),
      authors: [
        'https://railson.dev'
      ],
      tags: [
        // 'Tag A', 'Tag B', 'Tag C'
      ],
    },
  }
}
export default async function Page({params}: any) {
  // TODO: verify cache
  const data: any = await fetchData(params["post-uid"], params["post-timestamp"]);
  if (data.post == null) {
    notFound()
  }
  const { contentMarkdown} = data.post
  return (
    <>
      <main className="flex min-h-screen flex-col items-center justify-between p-24">
        <div className="mb-32 grid text-center lg:max-w-5xl lg:w-full lg:mb-0 lg:grid-cols-4 lg:text-left">
          <Markdown>{contentMarkdown.replace(/---(.|\n)+---/g, "")}</Markdown>
        </div>
      </main>
    </>
  );
}

async function fetchData(postUid: string, postTimestamp: string) {
  const query = gql`{
      post(uid: "${postUid}",updatingDateTime: "${new Date(Number(postTimestamp) * 1000).toISOString().replace(".000Z", "Z")}") {
        uid,
        timestamp,
        filePath,
        contentMarkdown
      }
    }`
  const client = new GraphQLClient(process.env.GRAPHQL_URL!)
  return await client.request(query)
}

export const revalidate = 60 * 60 // 1 hour