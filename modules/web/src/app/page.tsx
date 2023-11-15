import {gql, GraphQLClient} from "graphql-request";
import {Metadata} from "next";

type Post ={
  uid: string,
  timestamp:number,
  filePath:string,
  contentMarkdown:string
}
export const metadata: Metadata = {
  metadataBase: new URL('https://blog.railson.dev'),
  title: "Railson Dev Blog",
  description: "Made with ☕ by Rai",
  openGraph: {
    title: "Railson Dev Blog",
    description: "Made with ☕ by Rai",
    images: ['/seo.png'],
  },
}
export default async function Home() {
  const data = await fetchData();
  const posts = data.posts.filter((post,index,array)=>array.findIndex(x=>x.uid === post.uid) === index).sort((a, b) => a.timestamp - b.timestamp)
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="mb-32 grid text-center lg:max-w-5xl lg:w-full lg:mb-0 lg:grid-cols-4 lg:text-left">
        {posts.map((post:any) => {
          const {uid, timestamp, filePath, contentMarkdown} = post
          const title = contentMarkdown.match(/title: (.+)/)[1];
          const slug = filePath.replace(".md","");
          return (
            <a
              href={`/${slug}/${uid}/${timestamp}`}
              className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
              target="_blank"
              rel="noopener noreferrer"
            >
              <h2 className={`mb-3 text-2xl font-semibold`}>
                {title}{' '}
                <span
                  className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
              </h2>
              <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
                {slug}
              </p>
            </a>
          )
        })}

      </div>
    </main>
  )
}

async function fetchData() {
  const query = gql`{
      posts {
        uid,
        timestamp,
        filePath,
        contentMarkdown
      }
    }`
  const client = new GraphQLClient(process.env.GRAPHQL_URL!)
  return (await client.request(query)) as {posts: Array<Post>}
}

export const revalidate = 24 * 60 * 60 // 1 day