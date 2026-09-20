import type {ComponentType, ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import {Code2, Rocket, Zap} from 'lucide-react';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Icon: ComponentType<{className?: string; size?: number; strokeWidth?: number}>;
  description: ReactNode;
};

const featureList: FeatureItem[] = [
  {
    title: 'Zero Setup',
    Icon: Rocket,
    description: (
      <>
        Create and run a project in seconds with a simple CLI. No complex
        configuration, no boilerplate, just start building.
      </>
    ),
  },
  {
    title: 'JavaScript Scripting',
    Icon: Code2,
    description: (
      <>
        Write your game logic in JavaScript with built-in type definitions for
        autocomplete and a smooth developer experience.
      </>
    ),
  },
  {
    title: 'Lightweight & Fast',
    Icon: Zap,
    description: (
      <>
        Powered by a native Rust runtime, Oxid keeps startup fast and the
        footprint small without adding a heavy toolchain to the scripting flow.
      </>
    ),
  },
];

function Feature({title, Icon, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4', styles.featureColumn)}>
      <div className={styles.featureCard}>
        <div className={styles.iconWrap}>
          <Icon className={styles.featureIcon} size={36} strokeWidth={1.8} />
        </div>
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {featureList.map((props) => (
            <Feature key={props.title} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
