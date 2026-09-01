import type {Metadata} from 'next';
import { Plus_Jakarta_Sans, JetBrains_Mono } from 'next/font/google';
import './globals.css';

const sansFont = Plus_Jakarta_Sans({
  subsets: ['latin'],
  variable: '--font-sans',
  display: 'swap',
});

const monoFont = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-mono',
  display: 'swap',
});

export const metadata: Metadata = {
  title: 'PIRTM — Governed Computation Toolchain',
  description:
    'Phase-Indexed Recursive Tensor Mathematics and Multiplicity Operator Calculus (MOC) with Lean formal verification, MLIR backend, and spectral small-gain runtime enforcement.',
  openGraph: {
    title: 'PIRTM — Governed Computation Toolchain',
    description:
      'Formally verified, runtime-enforced programming language for safe, auditable software.',
    type: 'website',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'PIRTM — Governed Computation Toolchain',
    description:
      'Formally verified, runtime-enforced programming language for safe, auditable software.',
  },
};

export default function RootLayout({children}: {children: React.ReactNode}) {
  return (
    <html lang="en" className={`dark scroll-smooth ${sansFont.variable} ${monoFont.variable}`}>
      <body className="bg-[#080a0d] text-[#e6edf3] font-sans antialiased min-h-screen selection:bg-[#58a6ff]/30 selection:text-[#58a6ff]">
        {children}
      </body>
    </html>
  );
}
