import React from 'react';
import {gql, GraphQLClient} from "graphql-request";
import Markdown from "react-markdown";
import {notFound} from "next/navigation";
import Seo from "@/app/[post-slug]/[post-uid]/[post-timestamp]/seo";

export default async function Page({params}: any) {
  const data: any = await fetchData(params["post-uid"], params["post-timestamp"]);
  if (data.post == null) {
    notFound()
  }
  const {uid, timestamp, filePath, contentMarkdown} = data.post
  const title = contentMarkdown.match(/title: (.+)/)[1];
  const slug = filePath.replace(".md", "");
  return (
    <>
      <Seo
        uid={uid}
        timestamp={timestamp}
        title={title}
        slug={slug}
      />
      <main className="flex min-h-screen flex-col items-center justify-between p-24">
        <div className="mb-32 grid text-center lg:max-w-5xl lg:w-full lg:mb-0 lg:grid-cols-4 lg:text-left">
          <Markdown>{contentMarkdown.replace(/---(.|\n)+---/g, "").replaceAll(/<!--.+-->/g, "")}</Markdown>
        </div>
      </main>
    </>
  );
}

async function fetchData(postUid: string, postTimestamp: string) {
  const query = gql`{
      post(uid: "${postUid}",updatingDateTime: "${new Date(Number(postTimestamp) * 1000).toISOString().replace(".000Z", "+00:00")}") {
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