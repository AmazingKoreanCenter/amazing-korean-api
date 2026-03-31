interface CoverCardProps {
  imageSrc: string;
  imageAlt: string;
  title: string;
  subtitle: string;
  actionLabel: string;
  onClick: () => void;
}

export function CoverCard({
  imageSrc,
  imageAlt,
  title,
  subtitle,
  actionLabel,
  onClick,
}: CoverCardProps) {
  return (
    <button
      type="button"
      onClick={onClick}
      className="bg-card rounded-2xl overflow-hidden shadow-card hover:shadow-card-hover hover:-translate-y-1 transition-all duration-300 border hover:border-accent/50 text-left cursor-pointer"
    >
      <div className="aspect-[3/4] overflow-hidden bg-muted border-b">
        <img
          src={imageSrc}
          alt={imageAlt}
          className="w-full h-full object-cover"
          loading="lazy"
        />
      </div>
      <div className="p-4 space-y-2">
        <h3 className="font-semibold text-sm">{title}</h3>
        <p className="text-xs text-muted-foreground text-right py-0.5">{subtitle}</p>
        <span className="inline-flex items-center justify-center w-full rounded-md bg-primary text-primary-foreground text-sm font-medium h-8 px-3">
          {actionLabel}
        </span>
      </div>
    </button>
  );
}
