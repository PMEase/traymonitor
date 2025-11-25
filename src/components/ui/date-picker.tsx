import { ChevronDownIcon } from "lucide-react";
import * as React from "react";

import { Button } from "@/components/ui/button";
import { Calendar } from "@/components/ui/calendar";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

interface DatePickerProps {
  value?: Date;
  onChange?: (date: Date | undefined) => void;
  placeholder?: string;
  className?: string;
}

function DatePicker({
  value,
  onChange,
  placeholder = "Select date",
  className,
}: DatePickerProps) {
  const [open, setOpen] = React.useState(false);

  return (
    <Popover onOpenChange={setOpen} open={open}>
      <PopoverTrigger asChild>
        <Button
          className={`w-full justify-between font-normal ${className || ""}`}
          variant="outline"
        >
          {value ? value.toLocaleDateString() : placeholder}
          <ChevronDownIcon className="size-4" />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-auto overflow-hidden p-0">
        <Calendar
          captionLayout="dropdown"
          mode="single"
          onSelect={(date) => {
            onChange?.(date);
            setOpen(false);
          }}
          selected={value}
        />
        {value && (
          <div className="border-t p-2">
            <Button
              className="w-full"
              onClick={() => {
                onChange?.(undefined);
                setOpen(false);
              }}
              size="sm"
              variant="outline"
            >
              Clear
            </Button>
          </div>
        )}
      </PopoverContent>
    </Popover>
  );
}

export { DatePicker };
