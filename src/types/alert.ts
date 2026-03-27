export interface AlertRule {
  id: string;
  server_id: string;
  name: string;
  metric_key: string;
  condition: 'gt' | 'lt' | 'eq';
  threshold: number;
  duration_sec: number;
  enabled: boolean;
  created_at: number;
}

export interface AlertEvent {
  id: string;
  rule_id: string;
  server_id: string;
  status: 'firing' | 'resolved';
  message: string;
  fired_at: number;
  resolved_at: number | null;
}
