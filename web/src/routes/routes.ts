export type AppRoute = "play" | "practice" | "leaderboard" | "profile";

export const navRoutes: Array<{ id: AppRoute; label: string; path: string }> = [
  { id: "play", label: "Play", path: "/" },
  { id: "practice", label: "Practice", path: "/practice" },
  { id: "leaderboard", label: "Leaderboard", path: "/leaderboard" },
];

export function routeFromPath(pathname: string): AppRoute {
  switch (pathname.replace(/\/+$/, "") || "/") {
    case "/practice":
      return "practice";
    case "/leaderboard":
      return "leaderboard";
    case "/profile":
      return "profile";
    default:
      return "play";
  }
}

export function pathForRoute(route: AppRoute): string {
  if (route === "play") return "/";
  return `/${route}`;
}
