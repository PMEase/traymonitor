import { Spinner } from "@/components/ui/spinner";
import { cn } from "@/lib/utils";

export const Loading = ({
  message,
  className,
}: {
  message: string;
  className?: string;
}) => {
  return (
    <div className={cn("flex flex-col items-center gap-2", className)}>
      <Spinner className="size-10" />
      <p className="text-lg text-muted-foreground">{message}</p>
    </div>
  );
};
