import type { ReactNode } from "react";

interface FeatureItem {
  icon: ReactNode;
  title: string;
  description: string;
}

interface FeatureGridProps {
  items: FeatureItem[];
}

export function FeatureGrid({ items }: FeatureGridProps) {
  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
      {items.map((item) => (
        <div
          key={item.title}
          className="bg-card rounded-2xl p-5 md:p-8 shadow-card text-center hover:shadow-card-hover hover:-translate-y-1 hover:border-accent/50 transition-all duration-300 border"
        >
          <div className="w-16 h-16 rounded-2xl gradient-primary flex items-center justify-center mx-auto mb-6">
            {item.icon}
          </div>
          <h3 className="text-xl font-semibold mb-3">{item.title}</h3>
          <p className="text-muted-foreground leading-relaxed whitespace-pre-line">
            {item.description}
          </p>
        </div>
      ))}
    </div>
  );
}
