import { File } from "./file";
import { Metadata } from "./metadata";

type Node = File | Directory;

interface Directory {
  metadata: Metadata;
  children: Node[];
}
