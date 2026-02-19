/**
 * Shared pagination logic for list pages.
 */

export const ELLIPSIS: unique symbol = Symbol("ellipsis");

export type PageItem = number | typeof ELLIPSIS;

/**
 * Build a compact page-number array with ellipsis markers.
 *
 * @example getPageItems(5, 10) → [1, ELLIPSIS, 4, 5, 6, ELLIPSIS, 10]
 */
export function getPageItems(
  currentPage: number,
  totalPages: number,
  siblingCount = 1,
): PageItem[] {
  if (totalPages <= 5 + siblingCount * 2) {
    return Array.from({ length: totalPages }, (_, i) => i + 1);
  }

  const items: PageItem[] = [1];

  const start = Math.max(2, currentPage - siblingCount);
  const end = Math.min(totalPages - 1, currentPage + siblingCount);

  if (start > 2) {
    items.push(ELLIPSIS);
  }

  for (let page = start; page <= end; page += 1) {
    items.push(page);
  }

  if (end < totalPages - 1) {
    items.push(ELLIPSIS);
  }

  items.push(totalPages);
  return items;
}
