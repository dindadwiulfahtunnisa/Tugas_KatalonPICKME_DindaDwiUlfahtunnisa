<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>select_-- Pilih --BeginnerIntermediateAdvan_943818</name>
   <tag></tag>
   <elementGuidId>25f36c1d-1eb5-49e2-be8e-478cdad61f96</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='collapseLanguage--1']/div[3]/div[2]/select</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>select.custom-select.required.selectLanguageLevel--1--1</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#collapseLanguage--1 >> internal:role=combobox >> nth=0</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>select</value>
      <webElementGuid>1020402b-60ea-4e9a-bb54-2e8fa0ddb548</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-collapse</name>
      <type>Main</type>
      <value>#collapseLanguage--1</value>
      <webElementGuid>c7679ec3-ff27-4fae-b39c-d8fb21def195</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>custom-select required selectLanguageLevel--1--1</value>
      <webElementGuid>bd33799c-4c95-4156-8641-4cbe925d7377</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onchange</name>
      <type>Main</type>
      <value>
            if($(this).val() === ''){
                $('#txtLanguageLevel--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                $('#frame').contents().find('#txtListLanguageLevel--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                $('#txtListLanguageLevel--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
            } else {
                $('#txtLanguageLevel--1').text('('+$(this).find('option:selected').text()+')').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                $('#frame').contents().find('#txtListLanguageLevel--1').text('Writing         : '+$(this).find('option:selected').text()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                $('#txtListLanguageLevel--1').text('Writing        : '+$(this).find('option:selected').text()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                $('#frame').contents().find('#txtListLanguageLevel--1').text('Writing         : '+$(this).find('option:selected').text()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                $('#txtListLanguageLevel--1').text('Writing        : '+$(this).find('option:selected').text()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
            }
            isLanguageValueBlank(); autoSave(); validateRequire($(this))</value>
      <webElementGuid>3ae9fb5e-27cc-464d-850c-34f5acfa45ce</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            -- Pilih --BeginnerIntermediateAdvanceFluentNative
        </value>
      <webElementGuid>7d803722-1d04-4ccf-9951-b49083fe525f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;collapseLanguage--1&quot;)/div[@class=&quot;row&quot;]/div[@class=&quot;form-group col-md-6&quot;]/select[@class=&quot;custom-select required selectLanguageLevel--1--1&quot;]</value>
      <webElementGuid>e14902dc-1d1f-481b-a963-158d8b387942</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='collapseLanguage--1']/div[3]/div[2]/select</value>
      <webElementGuid>b163a899-3fb7-4942-ac23-88ee7d3d89fa</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Tingkat'])[1]/following::select[1]</value>
      <webElementGuid>cd509f50-e4f6-4c2d-bfe1-98af4d3b9be0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Kemampuan'])[1]/following::select[1]</value>
      <webElementGuid>ac1159fc-3fab-42b7-8e60-6be14a93fc54</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Kemampuan'])[2]/preceding::select[1]</value>
      <webElementGuid>25b0f597-20b3-47bf-96ae-5acbf82285bf</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Tingkat'])[2]/preceding::select[1]</value>
      <webElementGuid>e39ba599-d714-4176-8709-5ef1070b5c3d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div[3]/div[2]/select</value>
      <webElementGuid>ba6a8286-f57c-4499-9a79-b6b9e7eabfd0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//select[(text() = '
            -- Pilih --BeginnerIntermediateAdvanceFluentNative
        ' or . = '
            -- Pilih --BeginnerIntermediateAdvanceFluentNative
        ')]</value>
      <webElementGuid>4080ff07-7590-4625-8e7f-00c4fbbbe116</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
