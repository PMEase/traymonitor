import { useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";

export const AlertsView = () => {
  const navigate = useNavigate();
  return (
    <div className="h-screen bg-red-900 text-white">
      <h1 className="font-bold text-3xl">Alerts View</h1>
      <Button onClick={() => navigate("/")}>Back</Button>
    </div>
  );
};
