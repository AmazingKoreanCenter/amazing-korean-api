import { cn } from "@/lib/utils";

const sizeMap = {
  sm: "py-section-sm",
  md: "py-section-md",
  lg: "py-section-lg lg:py-hero-lg",
} as const;

const containerMap = {
  default: "max-w-[1350px]",
  narrow: "max-w-3xl",
} as const;

interface SectionContainerProps {
  size?: keyof typeof sizeMap;
  container?: keyof typeof containerMap;
  as?: React.ElementType;
  className?: string;
  children: React.ReactNode;
}

export function SectionContainer({
  size = "md",
  container = "default",
  as: Component = "section",
  className,
  children,
}: SectionContainerProps) {
  return (
    <Component className={cn(sizeMap[size], className)}>
      <div className={cn(containerMap[container], "mx-auto px-6 lg:px-8")}>
        {children}
      </div>
    </Component>
  );
}
