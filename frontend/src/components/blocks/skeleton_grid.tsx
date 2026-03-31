import React from "react";

import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { cn } from "@/lib/utils";

const gridCols = {
  2: "md:grid-cols-2",
  3: "md:grid-cols-3",
  4: "md:grid-cols-4",
} as const;

type SkeletonVariant = "video-card" | "content-card" | "study-card";

interface SkeletonGridProps {
  count: number;
  variant: SkeletonVariant;
  columns?: 2 | 3 | 4;
  className?: string;
}

function CardWithImageSkeleton() {
  return (
    <Card variant="elevated" className="overflow-hidden">
      <Skeleton className="aspect-video w-full" />
      <CardHeader className="space-y-2">
        <Skeleton className="h-6 w-3/4" />
      </CardHeader>
      <CardContent>
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-2/3 mt-2" />
      </CardContent>
    </Card>
  );
}

function StudyCardSkeleton() {
  return (
    <Card variant="elevated" className="overflow-hidden">
      <CardHeader className="space-y-3">
        <div className="flex items-center justify-between">
          <Skeleton className="h-6 w-24 rounded-full" />
          <Skeleton className="h-4 w-20" />
        </div>
        <Skeleton className="h-6 w-3/4" />
      </CardHeader>
      <CardContent>
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-2/3 mt-2" />
      </CardContent>
    </Card>
  );
}

const skeletonComponents: Record<SkeletonVariant, () => React.JSX.Element> = {
  "video-card": CardWithImageSkeleton,
  "content-card": CardWithImageSkeleton,
  "study-card": StudyCardSkeleton,
};

export function SkeletonGrid({
  count,
  variant,
  columns = 3,
  className,
}: SkeletonGridProps) {
  const SkeletonCard = skeletonComponents[variant];

  return (
    <div
      className={cn("grid grid-cols-1 gap-6", gridCols[columns], className)}
    >
      {Array.from({ length: count }, (_, index) => (
        <SkeletonCard key={`skeleton-${index}`} />
      ))}
    </div>
  );
}
