const MAX_SAMPLES = 2000;

export class LatencyRecorder {
  private samples: number[] = [];
  private sorted: number[] | null = null;

  record(ms: number): void {
    this.samples.push(ms);
    if (this.samples.length > MAX_SAMPLES) {
      this.samples.shift();
    }
    this.sorted = null;
  }

  clear(): void {
    this.samples = [];
    this.sorted = null;
  }

  get count(): number {
    return this.samples.length;
  }

  private ensureSorted(): number[] {
    if (this.sorted === null) {
      this.sorted = this.samples.slice().sort((a, b) => a - b);
    }
    return this.sorted;
  }

  percentile(p: number): number {
    const s = this.ensureSorted();
    if (s.length === 0) return 0;
    const idx = Math.ceil((p / 100) * s.length) - 1;
    return s[Math.max(0, idx)];
  }

  get p50(): number {
    return this.percentile(50);
  }
  get p95(): number {
    return this.percentile(95);
  }
  get p99(): number {
    return this.percentile(99);
  }
  get min(): number {
    const s = this.ensureSorted();
    return s.length > 0 ? s[0] : 0;
  }
  get max(): number {
    const s = this.ensureSorted();
    return s.length > 0 ? s[s.length - 1] : 0;
  }

  summary(): string {
    if (this.samples.length === 0) return "no samples";
    return (
      `n=${this.count} | ` +
      `p50=${this.p50.toFixed(1)}ms | ` +
      `p95=${this.p95.toFixed(1)}ms | ` +
      `p99=${this.p99.toFixed(1)}ms | ` +
      `min=${this.min.toFixed(1)} | ` +
      `max=${this.max.toFixed(1)}`
    );
  }
}
