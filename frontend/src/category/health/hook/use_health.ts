import { useQuery } from "@tanstack/react-query";

import { getHealth } from "@/api/health_api";

export const useHealth = () => {
  return useQuery({
    queryKey: ["health"],
    queryFn: getHealth,
    retry: false,
  });
};
