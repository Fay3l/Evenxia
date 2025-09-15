import React from "react";

type ButtonProps = {
  readonly children?: React.ReactNode;
  readonly variant?: "primary" | "secondary" | "tertiary";
  readonly onClick?: React.MouseEventHandler<HTMLButtonElement>;
};

export default function Button({ children = "", variant = "primary", onClick }: ButtonProps) {
  const styles = "px-4 py-2 rounded-md font-medium transition-all";
  const variants = {
    primary: "bg-blue-500 text-white hover:bg-blue-600",
    secondary: "bg-gray-500 text-white hover:bg-gray-600",
    tertiary: "bg-red-500 text-white hover:bg-red-600",
  };

  return (
    <button onClick={onClick} className={`${styles} ${variants[variant]}`}>
      {children}
    </button>
  );
}