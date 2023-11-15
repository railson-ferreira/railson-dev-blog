import React from 'react';
import {gql, GraphQLClient} from "graphql-request";
import {notFound, redirect} from "next/navigation";

type Post ={
  timestamp:number
}

export default async function Page({params}: any) {
  const slug = params["post-slug"];
  const uid =params["post-uid"];
  const data = await fetchData(uid);
  if (!data.postsByUid?.length) {
    notFound()
  }
  const postsTimestamps = data.postsByUid.map(x=>x.timestamp).sort((a,b)=>b-a)
  redirect(`/${slug}/${uid}/${postsTimestamps[0]}`)
}

async function fetchData(postUid: string) {
  const query = gql`{
      postsByUid(uid: "${postUid}") {
        timestamp
      }
    }`
  const client = new GraphQLClient(process.env.GRAPHQL_URL!)
  return (await client.request(query)) as {postsByUid: Array<Post>}
}

export const revalidate = 60 * 60 // 1 hour