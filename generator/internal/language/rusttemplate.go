// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package language

import (
	"strings"

	"github.com/googleapis/google-cloud-rust/generator/internal/api"
	"github.com/googleapis/google-cloud-rust/generator/internal/license"
	"github.com/iancoleman/strcase"
)

type RustTemplateData struct {
	Name              string
	Title             string
	Description       string
	PackageName       string
	SourcePackageName string
	PackageVersion    string
	RequiredPackages  []string
	HasServices       bool
	CopyrightYear     string
	BoilerPlate       []string
	Imports           []string
	DefaultHost       string
	Services          []*RustService
	Messages          []*RustMessage
	Enums             []*RustEnum
	NameToLower       string
	NotForPublication bool
}

type RustService struct {
	Methods             []*RustMethod
	NameToSnake         string
	NameToPascal        string
	ServiceNameToPascal string
	NameToCamel         string
	ServiceName         string
	DocLines            []string
	DefaultHost         string
}

type RustMessage struct {
	Fields             []*RustField
	BasicFields        []*RustField
	ExplicitOneOfs     []*RustOneOf
	NestedMessages     []*RustMessage
	Enums              []*RustEnum
	MessageAttributes  []string
	Name               string
	QualifiedName      string
	NameSnakeCase      string
	HasNestedTypes     bool
	DocLines           []string
	IsMap              bool
	IsPageableResponse bool
	PageableItem       *RustField
	ID                 string
	// The FQN is the source specification
	SourceFQN string
	// If true, this is a synthetic message, some generation is skipped for
	// synthetic messages
	HasSyntheticFields bool
}

type RustMethod struct {
	NameToSnake         string
	NameToCamel         string
	NameToPascal        string
	DocLines            []string
	InputTypeName       string
	OutputTypeName      string
	HTTPMethod          string
	HTTPMethodToLower   string
	HTTPPathFmt         string
	HTTPPathArgs        []string
	PathParams          []*RustField
	QueryParams         []*RustField
	HasBody             bool
	BodyAccessor        string
	IsPageable          bool
	ServiceNameToPascal string
	ServiceNameToCamel  string
	ServiceNameToSnake  string
	InputTypeID         string
	InputType           *RustMessage
	OperationInfo       *RustOperationInfo
}

type RustOperationInfo struct {
	MetadataType string
	ResponseType string
}

type RustOneOf struct {
	NameToPascal          string
	NameToSnake           string
	NameToSnakeNoMangling string
	FieldType             string
	DocLines              []string
	Fields                []*RustField
}

type RustField struct {
	NameToSnake           string
	NameToSnakeNoMangling string
	NameToCamel           string
	NameToPascal          string
	DocLines              []string
	FieldAttributes       []string
	FieldType             string
	PrimitiveFieldType    string
	JSONName              string
	AsQueryParameter      string
}

type RustEnum struct {
	Name          string
	NameSnakeCase string
	DocLines      []string
	Values        []*RustEnumValue
}

type RustEnumValue struct {
	DocLines []string
	Name     string
	Number   int32
	EnumType string
}

// newRustTemplateData creates a struct used as input for Mustache templates.
// Fields and methods defined in this struct directly correspond to Mustache
// tags. For example, the Mustache tag {{#Services}} uses the
// [Template.Services] field.
func newRustTemplateData(model *api.API, c *RustCodec) *RustTemplateData {
	c.LoadWellKnownTypes(model.State)
	data := &RustTemplateData{
		Name:              model.Name,
		Title:             model.Title,
		Description:       model.Description,
		PackageName:       c.PackageName(model),
		SourcePackageName: c.SourcePackageName(),
		PackageVersion:    c.PackageVersion(),
		HasServices:       len(model.Services) > 0,
		CopyrightYear:     c.CopyrightYear(),
		BoilerPlate: append(license.LicenseHeaderBulk(),
			"",
			" Code generated by sidekick. DO NOT EDIT."),
		Imports: c.Imports(),
		DefaultHost: func() string {
			if len(model.Services) > 0 {
				return model.Services[0].DefaultHost
			}
			return ""
		}(),
		Services: mapSlice(model.Services, func(s *api.Service) *RustService {
			return newRustService(s, c, model.State)
		}),
		Messages: mapSlice(model.Messages, func(m *api.Message) *RustMessage {
			return newRustMessage(m, c, model.State)
		}),
		Enums: mapSlice(model.Enums, func(e *api.Enum) *RustEnum {
			return newRustEnum(e, c, model.State)
		}),
		NameToLower:       strings.ToLower(model.Name),
		NotForPublication: c.NotForPublication(),
	}
	// Delay this until the Codec had a chance to compute what packages are
	// used.
	data.RequiredPackages = c.RequiredPackages()

	messagesByID := map[string]*RustMessage{}
	for _, m := range data.Messages {
		messagesByID[m.ID] = m
	}
	for _, s := range data.Services {
		for _, method := range s.Methods {
			if msg, ok := messagesByID[method.InputTypeID]; ok {
				method.InputType = msg
			} else if m, ok := model.State.MessageByID[method.InputTypeID]; ok {
				method.InputType = newRustMessage(m, c, model.State)
			}
		}
	}
	return data
}

func newRustService(s *api.Service, c *RustCodec, state *api.APIState) *RustService {
	// Some codecs skip some methods.
	methods := filterSlice(s.Methods, func(m *api.Method) bool {
		return c.GenerateMethod(m)
	})
	return &RustService{
		Methods: mapSlice(methods, func(m *api.Method) *RustMethod {
			return newRustMethod(m, c, state)
		}),
		NameToSnake:         c.ToSnake(s.Name),
		NameToPascal:        c.ToPascal(s.Name),
		ServiceNameToPascal: c.ToPascal(s.Name), // Alias for clarity
		NameToCamel:         c.ToCamel(s.Name),
		ServiceName:         s.Name,
		DocLines:            c.FormatDocComments(s.Documentation, state),
		DefaultHost:         s.DefaultHost,
	}
}

func newRustMessage(m *api.Message, c *RustCodec, state *api.APIState) *RustMessage {
	hasSyntheticFields := false
	for _, f := range m.Fields {
		if f.Synthetic {
			hasSyntheticFields = true
			break
		}
	}
	return &RustMessage{
		Fields: mapSlice(m.Fields, func(s *api.Field) *RustField {
			return newRustField(s, c, state)
		}),
		BasicFields: func() []*RustField {
			filtered := filterSlice(m.Fields, func(s *api.Field) bool {
				return !s.IsOneOf
			})
			return mapSlice(filtered, func(s *api.Field) *RustField {
				return newRustField(s, c, state)
			})
		}(),
		ExplicitOneOfs: mapSlice(m.OneOfs, func(s *api.OneOf) *RustOneOf {
			return newRustOneOf(s, c, state)
		}),
		NestedMessages: mapSlice(m.Messages, func(s *api.Message) *RustMessage {
			return newRustMessage(s, c, state)
		}),
		Enums: mapSlice(m.Enums, func(s *api.Enum) *RustEnum {
			return newRustEnum(s, c, state)
		}),
		MessageAttributes: c.MessageAttributes(m, state),
		Name:              c.MessageName(m, state),
		QualifiedName:     c.FQMessageName(m, state),
		NameSnakeCase:     c.ToSnake(m.Name),
		HasNestedTypes: func() bool {
			if len(m.Enums) > 0 || len(m.OneOfs) > 0 {
				return true
			}
			for _, child := range m.Messages {
				if !child.IsMap {
					return true
				}
			}
			return false
		}(),
		DocLines:           c.FormatDocComments(m.Documentation, state),
		IsMap:              m.IsMap,
		IsPageableResponse: m.IsPageableResponse,
		PageableItem:       newRustField(m.PageableItem, c, state),
		ID:                 m.ID,
		SourceFQN:          strings.TrimPrefix(m.ID, "."),
		HasSyntheticFields: hasSyntheticFields,
	}
}

func newRustMethod(m *api.Method, c *RustCodec, state *api.APIState) *RustMethod {
	method := &RustMethod{
		BodyAccessor:      c.BodyAccessor(m, state),
		DocLines:          c.FormatDocComments(m.Documentation, state),
		HTTPMethod:        m.PathInfo.Verb,
		HTTPMethodToLower: strings.ToLower(m.PathInfo.Verb),
		HTTPPathArgs:      c.HTTPPathArgs(m.PathInfo, state),
		HTTPPathFmt:       c.HTTPPathFmt(m.PathInfo, state),
		HasBody:           m.PathInfo.BodyFieldPath != "",
		InputTypeName:     c.MethodInOutTypeName(m.InputTypeID, state),
		NameToCamel:       strcase.ToCamel(m.Name),
		NameToPascal:      c.ToPascal(m.Name),
		NameToSnake:       strcase.ToSnake(m.Name),
		OutputTypeName:    c.MethodInOutTypeName(m.OutputTypeID, state),
		PathParams: mapSlice(PathParams(m, state), func(s *api.Field) *RustField {
			return newRustField(s, c, state)
		}),
		QueryParams: mapSlice(QueryParams(m, state), func(s *api.Field) *RustField {
			return newRustField(s, c, state)
		}),
		IsPageable:          m.IsPageable,
		ServiceNameToPascal: c.ToPascal(m.Parent.Name),
		ServiceNameToCamel:  c.ToCamel(m.Parent.Name),
		ServiceNameToSnake:  c.ToSnake(m.Parent.Name),
		InputTypeID:         m.InputTypeID,
	}
	if m.OperationInfo != nil {
		method.OperationInfo = &RustOperationInfo{
			MetadataType: c.MethodInOutTypeName(m.OperationInfo.MetadataTypeID, state),
			ResponseType: c.MethodInOutTypeName(m.OperationInfo.ResponseTypeID, state),
		}
	}
	return method
}

func newRustOneOf(oneOf *api.OneOf, c *RustCodec, state *api.APIState) *RustOneOf {
	return &RustOneOf{
		NameToPascal:          c.ToPascal(oneOf.Name),
		NameToSnake:           c.ToSnake(oneOf.Name),
		NameToSnakeNoMangling: c.ToSnakeNoMangling(oneOf.Name),
		FieldType:             c.OneOfType(oneOf, state),
		DocLines:              c.FormatDocComments(oneOf.Documentation, state),
		Fields: mapSlice(oneOf.Fields, func(field *api.Field) *RustField {
			return newRustField(field, c, state)
		}),
	}
}

func newRustField(field *api.Field, c *RustCodec, state *api.APIState) *RustField {
	if field == nil {
		return nil
	}
	return &RustField{
		NameToSnake:           c.ToSnake(field.Name),
		NameToSnakeNoMangling: c.ToSnakeNoMangling(field.Name),
		NameToCamel:           c.ToCamel(field.Name),
		NameToPascal:          c.ToPascal(field.Name),
		DocLines:              c.FormatDocComments(field.Documentation, state),
		FieldAttributes:       c.FieldAttributes(field, state),
		FieldType:             c.FieldType(field, state),
		PrimitiveFieldType:    c.PrimitiveFieldType(field, state),
		JSONName:              field.JSONName,
		AsQueryParameter:      c.AsQueryParameter(field, state),
	}
}

func newRustEnum(e *api.Enum, c *RustCodec, state *api.APIState) *RustEnum {
	return &RustEnum{
		Name:          c.EnumName(e, state),
		NameSnakeCase: c.ToSnake(c.EnumName(e, state)),
		DocLines:      c.FormatDocComments(e.Documentation, state),
		Values: mapSlice(e.Values, func(s *api.EnumValue) *RustEnumValue {
			return newRustEnumValue(s, e, c, state)
		}),
	}
}

func newRustEnumValue(ev *api.EnumValue, e *api.Enum, c *RustCodec, state *api.APIState) *RustEnumValue {
	return &RustEnumValue{
		DocLines: c.FormatDocComments(ev.Documentation, state),
		Name:     c.EnumValueName(ev, state),
		Number:   ev.Number,
		EnumType: c.EnumName(e, state),
	}
}
