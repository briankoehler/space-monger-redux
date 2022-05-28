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
