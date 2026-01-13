import { useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";

export const BuildsView = () => {
  const navigate = useNavigate();

  return (
    <div className="h-screen bg-zinc-900 text-white">
      <h1 className="font-bold text-3xl">Builds View</h1>
      <Button onClick={() => navigate("/settings")}>Settings</Button>
    </div>
  );
};
