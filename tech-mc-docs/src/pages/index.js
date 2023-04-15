import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';

import styles from './index.module.css';

function HomepageHeader() {
    const { siteConfig } = useDocusaurusContext();
    return (
        <header className={clsx('hero hero--primary', styles.heroBanner)}>
            <div className="container">
                <h1 className="hero__title">{siteConfig.title}</h1>
                <p className="hero__subtitle">{siteConfig.tagline}</p>
                <div className={styles.buttons}>
                    <Link
                        className="button button--secondary button--lg"
                        to="/docs/intro">
                        Docs
                    </Link>
                </div>
            </div>
        </header>
    );
}

export default function Home() {
    const { siteConfig } = useDocusaurusContext();
    return (
        <Layout
            title={`Home`}
            description={`${siteConfig.tagline}`}>
            <HomepageHeader />
            <main>
                <p className={styles.content + " container"}>
                    Currently this is just a documentation on how minecraft handels each tick. This is from a bunch of different resorcues from <a href='https://gist.github.com/pwouik/a3f5b4afcb3ff9ea0eeebb21b4a9ebdf' target='_blank'>this gist</a>. Go check that out for documentation outside the scope of this project.
                    <br></br>
                    <br></br>
                    This is mainly for redstone, but probably will contain stuff not for redstone. This will also include implementation details and why x was implementation the way.
                </p>
            </main>
        </Layout>
    );
}
