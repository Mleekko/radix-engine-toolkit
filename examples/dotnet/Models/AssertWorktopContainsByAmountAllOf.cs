/*
 * Transaction Lib
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

namespace Models
{
    /// <summary>
    /// AssertWorktopContainsByAmountAllOf
    /// </summary>
    [DataContract]
    public partial class AssertWorktopContainsByAmountAllOf : IEquatable<AssertWorktopContainsByAmountAllOf>, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="AssertWorktopContainsByAmountAllOf" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected AssertWorktopContainsByAmountAllOf() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="AssertWorktopContainsByAmountAllOf" /> class.
        /// </summary>
        /// <param name="resourceAddress">resourceAddress (required).</param>
        /// <param name="amount">amount (required).</param>
        public AssertWorktopContainsByAmountAllOf(ResourceAddress resourceAddress = default(ResourceAddress), Decimal amount = default(Decimal))
        {
            // to ensure "resourceAddress" is required (not null)
            if (resourceAddress == null)
            {
                throw new InvalidDataException("resourceAddress is a required property for AssertWorktopContainsByAmountAllOf and cannot be null");
            }
            else
            {
                this.ResourceAddress = resourceAddress;
            }

            // to ensure "amount" is required (not null)
            if (amount == null)
            {
                throw new InvalidDataException("amount is a required property for AssertWorktopContainsByAmountAllOf and cannot be null");
            }
            else
            {
                this.Amount = amount;
            }

        }

        /// <summary>
        /// Gets or Sets ResourceAddress
        /// </summary>
        [DataMember(Name = "resource_address", EmitDefaultValue = true)]
        public ResourceAddress ResourceAddress { get; set; }

        /// <summary>
        /// Gets or Sets Amount
        /// </summary>
        [DataMember(Name = "amount", EmitDefaultValue = true)]
        public Decimal Amount { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class AssertWorktopContainsByAmountAllOf {\n");
            sb.Append("  ResourceAddress: ").Append(ResourceAddress).Append("\n");
            sb.Append("  Amount: ").Append(Amount).Append("\n");
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
            return this.Equals(input as AssertWorktopContainsByAmountAllOf);
        }

        /// <summary>
        /// Returns true if AssertWorktopContainsByAmountAllOf instances are equal
        /// </summary>
        /// <param name="input">Instance of AssertWorktopContainsByAmountAllOf to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(AssertWorktopContainsByAmountAllOf input)
        {
            if (input == null)
                return false;

            return
                (
                    this.ResourceAddress == input.ResourceAddress ||
                    (this.ResourceAddress != null &&
                    this.ResourceAddress.Equals(input.ResourceAddress))
                ) &&
                (
                    this.Amount == input.Amount ||
                    (this.Amount != null &&
                    this.Amount.Equals(input.Amount))
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
                if (this.ResourceAddress != null)
                    hashCode = hashCode * 59 + this.ResourceAddress.GetHashCode();
                if (this.Amount != null)
                    hashCode = hashCode * 59 + this.Amount.GetHashCode();
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