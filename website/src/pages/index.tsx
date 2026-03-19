import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import useBaseUrl from '@docusaurus/useBaseUrl';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  const logoUrl = useBaseUrl('/img/oxid.svg');

  return (
    <header className={clsx(styles.heroBanner)}>
      <div className={clsx('container', styles.heroGrid)}>
        <div className={styles.heroCopy}>
          <div className={styles.brandLockup}>
            <img className={styles.heroLogo} src={logoUrl} alt="Oxid logo" />
            <span className={styles.eyebrow}>Rust runtime + JavaScript scripting</span>
          </div>
          <Heading as="h1" className={styles.heroTitle}>
            {siteConfig.tagline}
          </Heading>
          <p className={styles.heroSubtitle}>
            Build games in seconds, powered by a lightweight Rust runtime.
          </p>
          <div className={styles.buttons}>
            <Link className="button button--primary button--lg" to="/getting-started">
              Start Building
            </Link>
            <Link className="button button--secondary button--lg" to="/cli">
              Explore the CLI
            </Link>
          </div>
        </div>
        <div className={styles.heroPanel}>
          <div className={styles.panelHeader}>
            <span>main.js</span>
            <span>package.json</span>
          </div>
          <pre className={styles.codeBlock}>
            <code>{`import { GameObject } from "oxid/core";
import { Transform2D } from "oxid/math";
import { drawText } from "oxid/text";
import { Color } from "oxid/color";

export class MyApp extends GameObject {
  constructor() {
    super();
    this.position = new Transform2D(280, 220);
    this.color = new Color(1, 1, 1, 1);
  }

  onDraw() {
    drawText("Hello Oxid!", this.position, 32, this.color);
  }
}

export function main() {
  return new MyApp();
}`}</code>
          </pre>
        </div>
      </div>
    </header>
  );
}

function WorkflowSection() {
  return (
    <section className={styles.workflowSection}>
      <div className="container">
        <div className={styles.sectionHeading}>
          <Heading as="h2">CLI-first by design</Heading>
          <p>
            Oxid keeps the main workflow small: create a project, open the generated
            JavaScript entrypoint, and run it with the native runtime.
          </p>
        </div>
        <div className={styles.workflowGrid}>
          <div className={styles.workflowCard}>
            <span className={styles.step}>01</span>
            <Heading as="h3">Create</Heading>
            <p>
              <code>oxid new my-game</code> generates a minimal project with{' '}
              <code>main.js</code>, <code>package.json</code>, <code>oxid.d.ts</code>{' '}
              and <code>tsconfig.json</code>.
            </p>
          </div>
          <div className={styles.workflowCard}>
            <span className={styles.step}>02</span>
            <Heading as="h3">Script</Heading>
            <p>
              Write game logic in JavaScript using <code>GameObject</code>,
              lifecycle hooks and the built-in Oxid modules.
            </p>
          </div>
          <div className={styles.workflowCard}>
            <span className={styles.step}>03</span>
            <Heading as="h3">Run</Heading>
            <p>
              <code>oxid run</code> reads <code>package.json</code>, resolves{' '}
              <code>oxid.entry</code>, boots the JS runtime and opens the native
              window.
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();

  return (
    <Layout
      title={siteConfig.title}
      description="Zero-setup game engine with instant JavaScript scripting.">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        <WorkflowSection />
      </main>
    </Layout>
  );
}
