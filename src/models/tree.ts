import { isNode } from "./directory";

export interface Tree {
  root: File | Node;
}

export const isTree = (obj: any): obj is Tree => {
  return (
    typeof obj === "object" && obj !== null && "root" in obj && isNode(obj.root)
  );
};
