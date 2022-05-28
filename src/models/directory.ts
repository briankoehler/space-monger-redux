import { File, isFile } from "./file";
import { isMetadata, Metadata } from "./metadata";

type Node = File | Directory;

export interface Directory {
  metadata: Metadata;
  children: Node[];
}

export const isNode = (obj: any): obj is Node => {
  return (
    typeof obj === "object" && obj !== null && (isFile(obj) || isDirectory(obj))
  );
};

export const isDirectory = (obj: any): obj is Directory => {
  return (
    typeof obj === "object" &&
    obj !== null &&
    "metadata" in obj &&
    "children" in obj &&
    isMetadata(obj.metadata) &&
    obj.children.every(isNode)
  );
};
