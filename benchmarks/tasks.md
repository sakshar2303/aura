# Aura AI Agent Benchmark Suite

## Purpose

Prove the thesis: **AI coding agents produce better code faster in Aura than in TypeScript/Python/Swift.**

## Metrics

For each task, measure across Aura, TypeScript+React Native, Python+Kivy, and Swift+SwiftUI:

| Metric | Description | How Measured |
|---|---|---|
| **Token Count** | Total LLM tokens consumed (prompt + response) | Token counter on API calls |
| **First-Compile Success** | Does the first generated code compile without errors? | Boolean |
| **Iteration Count** | Number of generate→compile→fix cycles to working code | Counter |
| **Time to Working App** | Wall clock time from task start to passing all acceptance criteria | Seconds |
| **Code Quality Score** | Automated analysis: no security issues, no unused code, proper error handling | 0-100 |
| **Design Quality Score** | Does the output look professional without manual design intervention? | Human rating 1-5 |

## Task Definitions

### Tier 1: Simple Apps (should work first try in Aura)

**Task 1: Counter App**
- Description: App with a number display, increment/decrement buttons
- Acceptance: Number displays, buttons work, starts at 0
- Complexity: Minimal (1 screen, 1 state, 2 actions)

**Task 2: Hello Name**
- Description: Text input + greeting display. Type a name, see "Hello, {name}!"
- Acceptance: Input works, greeting updates live
- Complexity: Minimal (1 screen, 1 state, 1 binding)

**Task 3: Light/Dark Toggle**
- Description: App that switches between light and dark theme
- Acceptance: Toggle works, theme changes visually
- Complexity: Minimal (1 screen, 1 state, theme switching)

**Task 4: Static Profile Card**
- Description: A profile card with avatar, name, bio, and 3 stat counters
- Acceptance: Correct layout, proper spacing, professional look
- Complexity: Low (1 screen, no state, layout + design)

**Task 5: Timer**
- Description: Countdown timer with start/pause/reset buttons
- Acceptance: Timer counts down, buttons work correctly, shows MM:SS
- Complexity: Low (1 screen, 2 states, 3 actions, time formatting)

### Tier 2: Standard Apps (should work in 1-2 iterations in Aura)

**Task 6: Todo List**
- Description: Add todos, mark complete, delete, filter (all/active/done)
- Acceptance: All CRUD operations work, filter works, empty state shown
- Complexity: Medium (1 screen, 3 states, 4 actions, 1 component)

**Task 7: Weather Display**
- Description: Show current weather with temperature, condition icon, and 5-day forecast
- Acceptance: Data displays correctly, condition icons match, responsive
- Complexity: Medium (1 screen, mock data, 2 components, responsive grid)

**Task 8: Settings Screen**
- Description: Settings page with toggles, picker, slider, and sections
- Acceptance: All input types work, values persist in state
- Complexity: Medium (1 screen, 5+ states, multiple input types)

**Task 9: Photo Gallery**
- Description: Grid of photos with tap-to-enlarge modal
- Acceptance: Grid layout, tap works, modal shows full image, dismiss works
- Complexity: Medium (2 screens/modals, grid layout, navigation)

**Task 10: Notes App**
- Description: List of notes with add/edit/delete, rich text preview
- Acceptance: CRUD works, text formatting preserved, sorted by date
- Complexity: Medium (2 screens, navigation, text handling)

### Tier 3: Complex Apps (should work in 2-3 iterations in Aura)

**Task 11: Chat Interface**
- Description: Chat UI with message bubbles, input, sent/received styling
- Acceptance: Messages display correctly, mine vs theirs styled differently, auto-scroll
- Complexity: High (2 screens, chat layout, user context, scroll behavior)

**Task 12: E-Commerce Product Page**
- Description: Product detail with image carousel, size picker, add to cart, reviews
- Acceptance: Carousel works, size selection, cart updates, review display
- Complexity: High (1 screen, multiple components, complex state)

**Task 13: Dashboard with Charts**
- Description: Analytics dashboard with stat cards, bar chart, line chart, date picker
- Acceptance: Data visualizes correctly, date range filtering works
- Complexity: High (1 screen, data visualization, responsive layout)

**Task 14: Multi-Step Form**
- Description: 3-step registration: personal info → preferences → confirmation
- Acceptance: Navigation between steps, validation, back support, summary
- Complexity: High (3 screens/steps, form validation, navigation)

**Task 15: Music Player**
- Description: Now playing screen with album art, progress slider, controls, playlist
- Acceptance: Controls respond, progress slider works, playlist navigation
- Complexity: High (2 screens, slider, animation, complex layout)

### Tier 4: Full Apps (Aura should be significantly better)

**Task 16: Social Media Feed**
- Description: Infinite scroll feed with posts, likes, comments, user profiles
- Acceptance: Feed renders, likes toggle, comment thread works, profile navigation
- Complexity: Very High (4+ screens, complex state, nested navigation)

**Task 17: Kanban Board**
- Description: Trello-like board with columns, draggable cards, add/edit/delete
- Acceptance: Columns render, cards can be moved, CRUD on cards/columns
- Complexity: Very High (drag-and-drop, complex state, responsive)

**Task 18: Calendar App**
- Description: Month/week/day views, event creation, event detail, reminders
- Acceptance: Calendar renders correctly, events display, date navigation
- Complexity: Very High (3 views, date math, complex layout, modal)

**Task 19: Cross-Platform Fitness Tracker**
- Description: Track workouts, display stats, weekly progress chart, dark mode
- Acceptance: Works on iOS + Android + Web from same source, data persists
- Complexity: Very High (3 platforms, charts, persistent state, theme)

**Task 20: Full E-Commerce App**
- Description: Browse → product detail → cart → checkout flow with auth
- Acceptance: Full flow works, cart persists, navigation stack correct, security types
- Complexity: Maximum (5+ screens, auth, security types, complex navigation)

## Benchmark Protocol

1. **Agent Setup**: Each agent gets the task description only (no example code)
2. **Language Setup**: Each language gets its standard project scaffold
3. **Measurement Start**: Timer begins when agent receives the task
4. **Iteration Loop**: Agent generates code → compile → if errors, agent sees errors and iterates
5. **Measurement End**: Timer stops when ALL acceptance criteria pass
6. **Post-hoc Analysis**: Code quality score + design quality score assessed

## Expected Results

If Aura is successful, we expect:
- **Tier 1**: 100% first-compile success in Aura vs ~60% in TypeScript
- **Tier 2**: 1.2 average iterations in Aura vs 2.5 in TypeScript
- **Tier 3**: 2x fewer tokens in Aura vs TypeScript
- **Tier 4**: Aura produces working cross-platform apps; TypeScript requires separate React Native + CSS
- **Design Quality**: Aura apps score 4+ without manual design; TypeScript apps score 2-3

## Running Benchmarks

```
aura benchmark run --agent claude --task all
aura benchmark run --agent gpt4 --task 1-10
aura benchmark results --compare typescript
```
