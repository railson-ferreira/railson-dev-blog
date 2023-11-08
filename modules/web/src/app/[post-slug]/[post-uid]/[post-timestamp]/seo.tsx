"use client"
import React from 'react';
import { NextSeo } from 'next-seo';

function Seo({title, slug, uid, timestamp}:any) {
  return (

    <NextSeo
      title={title}
      canonical={`https://blog.railson.dev/${slug}/${uid}/${timestamp}`}
      openGraph={{
        title: title,
        description: title,
        url: `https://blog.railson.dev/${slug}/${uid}/${timestamp}`,
        type: 'article',
        article: {
          publishedTime: new Date(Number(uid)*1000).toISOString(),
          modifiedTime: new Date(Number(timestamp)*1000).toISOString(),
          authors: [
            'https://railson.dev'
          ],
          tags: [
            // 'Tag A', 'Tag B', 'Tag C'
          ],
        },
        images: [
          // {
          //   url: 'https://www.test.ie/images/cover.jpg',
          //   width: 850,
          //   height: 650,
          //   alt: 'Photo of text',
          // },
        ],
      }}
    />
  );
}

export default Seo;