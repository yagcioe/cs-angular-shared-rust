import { ChangeDetectionStrategy, Component, type OnInit, signal } from '@angular/core';
import init, { get_factorial, greet } from 'wasm-example';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AppComponent {
  jsResult = signal<string>('');
  rsResult = signal<string>('');
  jsTime = signal<string>('');
  rsTime = signal<string>('');
  calculating = signal<boolean>(false);

  calculate(inp: number | string) {
    greet();
    this.calculating.set(true);

    setTimeout(() => {
      const n = typeof inp === 'number' ? inp : parseInt(inp, 10);
      const jsTimeStart = performance.now();
      let f = 0;
      for (let i = 0; i < 10000000; i++) {
        f = factorial(n);
      }
      this.jsResult.set(f.toString());
      this.jsTime.set(((performance.now() - jsTimeStart) / 1000).toFixed(4) + 's');

      const rsTimeStart = performance.now();
      this.rsResult.set(get_factorial(n));
      this.rsTime.set(((performance.now() - rsTimeStart) / 1000).toFixed(4) + 's');

      this.calculating.set(false);
    }, 50);
  }
}


function factorial(x: number): number {
  if (x === 0) {
    return 1;
  } else {
    return x * factorial(x - 1);
  }
}