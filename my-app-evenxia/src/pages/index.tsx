import Image from "next/image";
import { Geist, Geist_Mono } from "next/font/google";
import Button from "@/components/Button";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export default function Home() {
  return (
    <div>
      <header className="flex flex-row justify-between m-10">
        <p>Evenxia</p>
        <div className="flex flex-row gap-4">
          <Button>Login</Button>
          <Button>Sign Up</Button>
        </div>
      </header>
    </div>
  );
}
