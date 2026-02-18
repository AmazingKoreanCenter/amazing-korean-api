import { cn } from "@/lib/utils";

interface HeroSectionProps {
  badge?: React.ReactNode;
  title: React.ReactNode;
  subtitle?: React.ReactNode;
  size?: "default" | "sm";
  className?: string;
  children?: React.ReactNode;
}

export function HeroSection({
  badge,
  title,
  subtitle,
  size = "default",
  className,
  children,
}: HeroSectionProps) {
  return (
    <section
      className={cn("relative overflow-hidden bg-hero-gradient", className)}
    >
      {/* Decorative blobs */}
      <div className="absolute inset-0 overflow-hidden">
        <div className="absolute -top-40 -right-40 w-80 h-80 bg-accent/10 rounded-full blur-3xl" />
        <div className="absolute -bottom-40 -left-40 w-80 h-80 bg-secondary/10 rounded-full blur-3xl" />
      </div>

      <div
        className={cn(
          "relative max-w-[1350px] mx-auto px-6 lg:px-8",
          size === "default" ? "py-section-lg lg:py-hero-lg" : "py-section-md lg:py-section-lg",
        )}
      >
        <div className="max-w-3xl mx-auto text-center">
          {badge && (
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-background shadow-sm border mb-8">
              {badge}
            </div>
          )}

          <h1
            className={cn(
              "font-bold tracking-tight mb-4 break-keep",
              size === "default"
                ? "text-4xl md:text-5xl lg:text-6xl mb-6"
                : "text-4xl md:text-5xl",
            )}
          >
            {title}
          </h1>

          {subtitle && (
            <p className="text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto leading-relaxed">
              {subtitle}
            </p>
          )}

          {children}
        </div>
      </div>
    </section>
  );
}
