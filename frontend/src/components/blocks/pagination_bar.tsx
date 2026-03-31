import { useMemo } from "react";

import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";
import { ELLIPSIS, getPageItems } from "@/lib/pagination";
import { cn } from "@/lib/utils";

interface PaginationBarProps {
  currentPage: number;
  totalPages: number;
  onPageChange: (page: number) => void;
  className?: string;
}

export function PaginationBar({
  currentPage,
  totalPages,
  onPageChange,
  className,
}: PaginationBarProps) {
  const pageItems = useMemo(
    () => getPageItems(currentPage, totalPages),
    [currentPage, totalPages],
  );

  const hasPrev = currentPage > 1;
  const hasNext = currentPage < totalPages;

  const handleChange = (page: number) => {
    if (page === currentPage || page < 1 || page > totalPages) return;
    onPageChange(page);
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  if (totalPages <= 1) return null;

  return (
    <div className={cn("mt-12 flex justify-center", className)}>
      <Pagination>
        <PaginationContent className="gap-1">
          <PaginationItem>
            <PaginationPrevious
              href="#"
              onClick={(e) => {
                e.preventDefault();
                if (hasPrev) handleChange(currentPage - 1);
              }}
              aria-disabled={!hasPrev}
              className={`rounded-xl ${!hasPrev ? "pointer-events-none opacity-50" : ""}`}
            />
          </PaginationItem>

          {pageItems.map((item, index) => (
            <PaginationItem
              key={item === ELLIPSIS ? `ellipsis-${index}` : item}
            >
              {item === ELLIPSIS ? (
                <PaginationEllipsis />
              ) : (
                <PaginationLink
                  href="#"
                  isActive={item === currentPage}
                  onClick={(e) => {
                    e.preventDefault();
                    handleChange(item);
                  }}
                  className={`rounded-xl ${item === currentPage ? "gradient-primary text-white border-0" : ""}`}
                >
                  {item}
                </PaginationLink>
              )}
            </PaginationItem>
          ))}

          <PaginationItem>
            <PaginationNext
              href="#"
              onClick={(e) => {
                e.preventDefault();
                if (hasNext) handleChange(currentPage + 1);
              }}
              aria-disabled={!hasNext}
              className={`rounded-xl ${!hasNext ? "pointer-events-none opacity-50" : ""}`}
            />
          </PaginationItem>
        </PaginationContent>
      </Pagination>
    </div>
  );
}
