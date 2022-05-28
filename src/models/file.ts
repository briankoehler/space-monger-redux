import { isMetadata, Metadata } from "./metadata";

export interface File {
  metadata: Metadata;
}

export const isFile = (obj: any): obj is File => {
  return (
    typeof obj === "object" &&
    obj !== null &&
    "metadata" in obj &&
    isMetadata(obj.metadata)
  );
};
