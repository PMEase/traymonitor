import React from "react";
import { IconBase } from "./icon";

export const CancelledIcon = React.forwardRef<
  SVGSVGElement,
  React.SVGProps<SVGSVGElement>
>((props, ref) => {
  return (
    <IconBase
      {...props}
      aria-hidden="true"
      ref={ref}
      version="1.1"
      viewBox="0 0 1024 1024"
    >
      <path d="M512 0C229.257143 0 0 229.257143 0 512s229.257143 512 512 512 512-229.257143 512-512S794.742857 0 512 0z m268.342857 841.714286L182.285714 243.657143c18.285714-22.514286 38.857143-43.085714 61.371429-61.371429l598.057143 598.057143c-18.285714 22.4-38.857143 43.085714-61.371429 61.371429z" />
    </IconBase>
  );
});

export const FailedIcon = React.forwardRef<
  SVGSVGElement,
  React.SVGProps<SVGSVGElement>
>((props, ref) => {
  return (
    <IconBase
      aria-hidden="true"
      version="1.1"
      {...props}
      ref={ref}
      viewBox="0 0 16 16"
    >
      <path
        d="M2.343 13.657A8 8 0 1113.657 2.343 8 8 0 012.343 13.657zM6.03 4.97a.75.75 0 00-1.06 1.06L6.94 8 4.97 9.97a.75.75 0 101.06 1.06L8 9.06l1.97 1.97a.75.75 0 101.06-1.06L9.06 8l1.97-1.97a.75.75 0 10-1.06-1.06L8 6.94 6.03 4.97z"
        fill-rule="evenodd"
      />
    </IconBase>
  );
});
