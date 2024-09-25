import { ChangeDetectionStrategy, Component, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { add_one, get_factorial } from 'wasm-example';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AppComponent {
  addResult = signal<number>(0);

  jsResult = signal<string>('');
  rsResult = signal<string>('');
  jsTime = signal<string>('');
  rsTime = signal<string>('');
  calculating = signal<boolean>(false);

  calculate(inp: number | string) {
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

  protected add(value: string) {
    const n = typeof value === 'number' ? value : parseInt(value, 10);
    this.addResult.set(add_one(n))
  }

}

function factorial(x: number): number {
  if (x === 0) {
    return 1;
  } else {
    return x * factorial(x - 1);
  }
}