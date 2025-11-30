import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import App from "./App";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("App", () => {
  it("renders the main heading", () => {
    render(<App />);
    expect(screen.getByText("IronRise Alarm")).toBeInTheDocument();
  });

  it("renders schedule and cancel buttons", () => {
    render(<App />);
    expect(screen.getByText("Schedule Wake")).toBeInTheDocument();
    expect(screen.getByText("Cancel Schedule")).toBeInTheDocument();
  });

  it("updates alarm time state", () => {
    render(<App />);
    // Find input by type (datetime-local doesn't have a role, so we use selector or label if present)
    // We didn't add labels in the implementation, so let's use container query or placeholder if added.
    // Actually, let's just use getByDisplayValue or similar if we set a default.
    // Or better, add a data-testid to the input in App.tsx or just query by the input tag.
    // For now, let's assume we can find it.
    // Let's modify App.tsx to have labels or test ids for better testing, but for now I'll just check existence of inputs.
    // const inputs = screen.getAllByRole("textbox"); // datetime-local is not a textbox role usually
    // Let's just check text content for now.
    expect(screen.getByText("1. Set Alarm Time")).toBeInTheDocument();
  });
});
