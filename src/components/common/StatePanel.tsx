import type { ReactNode } from 'react';

interface StatePanelProps {
  eyebrow?: string;
  title: string;
  description: string;
  action?: ReactNode;
}

export default function StatePanel({ eyebrow, title, description, action }: StatePanelProps) {
  return (
    <div className="glass-panel state-panel rounded-[28px] border p-8 shadow-2xl">
      {eyebrow ? (
        <p className="shell-kicker mb-3">
          {eyebrow}
        </p>
      ) : null}
      <h2 className="state-panel-title">{title}</h2>
      <p className="state-panel-body mt-3 max-w-2xl text-sm leading-7">{description}</p>
      {action ? <div className="mt-6">{action}</div> : null}
    </div>
  );
}
