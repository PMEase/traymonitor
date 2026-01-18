import type { ReactElement } from "react";
import { cn } from "@/lib/utils";

export type SVGAttributes = Partial<React.SVGProps<SVGSVGElement>>;
type ComponentAttributes = React.RefAttributes<SVGSVGElement> & SVGAttributes;

export interface IconProps
  extends Omit<
    ComponentAttributes,
    "children"
  > /* React.SVGProps<SVGSVGElement> */ {
  size?: number | string;
  title?: string;
}

export type Icon = React.ForwardRefExoticComponent<IconProps>;

export function IconBase(
  props: IconProps & { children: React.ReactNode }
): ReactElement {
  const { size, title, className, children, width, height, ...svgProps } =
    props;
  const computedSize = size ?? "1em";
  const classes = cn("svgicon", className);
  const resolvedWidth = width ?? computedSize;
  const resolvedHeight = height ?? computedSize;

  return (
    <svg
      aria-label={title}
      fill="currentColor"
      stroke="currentColor"
      strokeWidth="0"
      // {...attr}
      {...svgProps}
      className={classes}
      height={resolvedHeight}
      width={resolvedWidth}
      xmlns="http://www.w3.org/2000/svg"
    >
      <title>{title ?? "SVG Icon"}</title>
      {children}
    </svg>
  );
}
