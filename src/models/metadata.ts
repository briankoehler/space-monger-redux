interface SystemTime {
  nanos_since_epoch: number;
  secs_since_epoch: number;
}

export interface Metadata {
  name: string;
  size: number;
  modified?: SystemTime;
  accessed?: SystemTime;
  created?: SystemTime;
}

export const isMetadata = (obj: any): obj is Metadata => {
  return (
    typeof obj === "object" &&
    obj !== null &&
    "name" in obj &&
    "size" in obj &&
    typeof obj.name === "string" &&
    typeof obj.size === "number" &&
    (obj.modified === undefined || isSystemTime(obj.modified)) &&
    (obj.accessed === undefined || isSystemTime(obj.accessed)) &&
    (obj.created === undefined || isSystemTime(obj.created))
  );
};

const isSystemTime = (obj: any): obj is SystemTime => {
  return (
    typeof obj === "object" &&
    obj !== null &&
    "nanos_since_epoch" in obj &&
    "secs_since_epoch" in obj &&
    typeof obj.nanos_since_epoch === "number" &&
    typeof obj.secs_since_epoch === "number"
  );
};
