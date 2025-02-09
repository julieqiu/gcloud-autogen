package gcloud

import (
	"bytes"
	"fmt"
	"os"
	"strings"
	"text/template"
)

// Command represents a single command with a name and a wrapped description.
type Command struct {
	Name         string
	Short        string
	WrappedShort []string
}

// HelpData defines the data to populate the usage template.
type HelpData struct {
	Description string
	Commands    []Command
}

// wrapShort wraps the short description into lines of a fixed length (50 characters).
func wrapShort(short string) string {
	const lineLength = 50
	var wrapped string
	words := strings.Fields(short)

	var line string
	for _, word := range words {
		// If adding the word exceeds the line length, start a new line
		if len(line)+len(word)+1 > lineLength {
			wrapped += line + "\n"
			line = word
		} else {
			if line != "" {
				line += " "
			}
			line += word
		}
	}

	// Append any remaining line
	if line != "" {
		wrapped += line
	}

	return wrapped
}

// prepareCommands processes the commands, wrapping their descriptions.
func prepareCommands(commands []Command) []Command {
	for i := range commands {
		commands[i].WrappedShort = strings.Split(wrapShort(commands[i].Short), "\n")
	}
	return commands
}

func Run() error {
	// Example list of commands
	commands := []Command{
		{
			"init",
			"Initialize the cloud environment. This command sets up the basic configuration for your Google Cloud project.",
			nil,
		},
		{
			"version",
			"Print version information for Google Cloud CLI",
			nil,
		},
		{
			"secrets",
			"Manage secrets on Google Cloud.",
			nil,
		},
	}

	// Process commands to wrap their descriptions
	commands = prepareCommands(commands)

	// Read the usage template from the file
	usageTemplateFile := "templates/help.txt"
	usageTemplate, err := os.ReadFile(usageTemplateFile)
	if err != nil {
		return err
	}

	// Populate the HelpData struct
	helpData := HelpData{
		Description: "gcloud is a tool for managing Google Cloud services and resources.",
		Commands:    commands,
	}

	// Capture the rendered output into a buffer
	var buf bytes.Buffer
	tmpl := template.Must(template.New("usage").Parse(string(usageTemplate)))
	err = tmpl.Execute(&buf, helpData)
	if err != nil {
		return err
	}

	// Print the final output
	fmt.Print(buf.String())
	return nil
}
