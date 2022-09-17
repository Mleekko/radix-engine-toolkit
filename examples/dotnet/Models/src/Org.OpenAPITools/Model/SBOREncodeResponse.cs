/*
 * Transaction Library
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */

using System;
using System.Linq;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.ComponentModel.DataAnnotations;
using OpenAPIDateConverter = Org.OpenAPITools.Client.OpenAPIDateConverter;

namespace Org.OpenAPITools.Model
{
    /// <summary>
    /// The response to the &#x60;SBOREncodeRequest&#x60; containing the hex-encoded representation of the SBOR-encoded &#x60;Value&#x60;
    /// </summary>
    [DataContract]
    public partial class SBOREncodeResponse :  IEquatable<SBOREncodeResponse>, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="SBOREncodeResponse" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected SBOREncodeResponse() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="SBOREncodeResponse" /> class.
        /// </summary>
        /// <param name="encodedValue">The hex-encoded representation of the SBOR-encoded &#x60;Value&#x60; (required).</param>
        public SBOREncodeResponse(string encodedValue = default(string))
        {
            // to ensure "encodedValue" is required (not null)
            if (encodedValue == null)
            {
                throw new InvalidDataException("encodedValue is a required property for SBOREncodeResponse and cannot be null");
            }
            else
            {
                this.EncodedValue = encodedValue;
            }

        }

        /// <summary>
        /// The hex-encoded representation of the SBOR-encoded &#x60;Value&#x60;
        /// </summary>
        /// <value>The hex-encoded representation of the SBOR-encoded &#x60;Value&#x60;</value>
        [DataMember(Name="encoded_value", EmitDefaultValue=true)]
        public string EncodedValue { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class SBOREncodeResponse {\n");
            sb.Append("  EncodedValue: ").Append(EncodedValue).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public virtual string ToJson()
        {
            return Newtonsoft.Json.JsonConvert.SerializeObject(this, Newtonsoft.Json.Formatting.Indented);
        }

        /// <summary>
        /// Returns true if objects are equal
        /// </summary>
        /// <param name="input">Object to be compared</param>
        /// <returns>Boolean</returns>
        public override bool Equals(object input)
        {
            return this.Equals(input as SBOREncodeResponse);
        }

        /// <summary>
        /// Returns true if SBOREncodeResponse instances are equal
        /// </summary>
        /// <param name="input">Instance of SBOREncodeResponse to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(SBOREncodeResponse input)
        {
            if (input == null)
                return false;

            return 
                (
                    this.EncodedValue == input.EncodedValue ||
                    (this.EncodedValue != null &&
                    this.EncodedValue.Equals(input.EncodedValue))
                );
        }

        /// <summary>
        /// Gets the hash code
        /// </summary>
        /// <returns>Hash code</returns>
        public override int GetHashCode()
        {
            unchecked // Overflow is fine, just wrap
            {
                int hashCode = 41;
                if (this.EncodedValue != null)
                    hashCode = hashCode * 59 + this.EncodedValue.GetHashCode();
                return hashCode;
            }
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<System.ComponentModel.DataAnnotations.ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            yield break;
        }
    }

}