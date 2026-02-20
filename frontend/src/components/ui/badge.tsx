import * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

const badgeVariants = cva(
  "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2",
  {
    variants: {
      variant: {
        default: "border-transparent bg-primary text-primary-foreground shadow",
        secondary: "border-transparent bg-secondary text-secondary-foreground",
        destructive:
          "border-transparent bg-destructive text-destructive-foreground",
        outline: "text-foreground",
        success: "border-transparent bg-status-success text-status-success-foreground",
        warning: "border-transparent bg-status-warning text-status-warning-foreground",
        info: "border-transparent bg-status-info text-status-info-foreground",
        // Badge-only fixed colors (theme-independent for enum display)
        blue: "border-transparent bg-badge-blue text-badge-blue-foreground",
        orange: "border-transparent bg-badge-orange text-badge-orange-foreground",
        purple: "border-transparent bg-badge-purple text-badge-purple-foreground",
        yellow: "border-transparent bg-badge-yellow text-badge-yellow-foreground",
        sky: "border-transparent bg-badge-sky text-badge-sky-foreground",
        indigo: "border-transparent bg-badge-indigo text-badge-indigo-foreground",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
);

export interface BadgeProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof badgeVariants> {}

function Badge({ className, variant, ...props }: BadgeProps) {
  return (
    <div className={cn(badgeVariants({ variant }), className)} {...props} />
  );
}

export { Badge, badgeVariants };
